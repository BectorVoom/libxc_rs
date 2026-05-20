//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 555/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk555<F: Float>(t45: F, t57: F, t150: F, t1531: F, t190: F, t162: F, t187: F, t1469: F, t766: F, t770: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1532 = t150 * t1531;
    let t1533 = t1532 * t190;
    let t1534 = t1531 * t162;
    let t1536 = F::cast_from(0.19751673498613801407e-1_f64) * t1534 * t187;
    let t1539 = piecewise3::<F>(t151, F::new(0.0), F::new(2.0) / F::new(3.0) * t766 * t1469);
    let t1542 = piecewise3::<F>(t155, F::new(0.0), -F::new(2.0) / F::new(3.0) * t770 * t1469);
    let t1544 = t1539 / F::new(2.0) + t1542 / F::new(2.0);
    (t1532, t1533, t1534, t1536, t1544)
}

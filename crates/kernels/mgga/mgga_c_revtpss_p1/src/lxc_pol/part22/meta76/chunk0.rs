//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 554/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk554<F: Float>(t45: F, t57: F, t1522: F, t706: F, t1469: F, t78: F, t81: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1524 = F::new(4.0) * t706 * t1522;
    let t1527 = piecewise3::<F>(t151, F::new(0.0), F::new(4.0) / F::new(3.0) * t78 * t1469);
    let t1530 = piecewise3::<F>(t155, F::new(0.0), -F::new(4.0) / F::new(3.0) * t81 * t1469);
    let t1531 = t1527 + t1530;
    (t1524, t1531)
}

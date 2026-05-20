//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 386/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk386<F: Float>(t45: F, t57: F, t1518: F, t508: F, t1469: F, t190: F, t706: F, t78: F, t81: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1519 = t508 * t1518;
    let t1522 = t190 * t1469;
    let t1524 = F::new(4.0) * t706 * t1522;
    let t1527 = piecewise3::<F>(t151, F::new(0.0), F::new(4.0) / F::new(3.0) * t78 * t1469);
    let t1530 = piecewise3::<F>(t155, F::new(0.0), -F::new(4.0) / F::new(3.0) * t81 * t1469);
    let t1531 = t1527 + t1530;
    (t1519, t1522, t1524, t1531)
}

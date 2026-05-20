//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 509/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk509<F: Float>(t1469: F, t48: F, t51: F, t53: F, rho1: F, sigma2: F) -> (F, F, F, F) {
    let t1474 = t48 * t1469;
    let t1477 = t51 * rho1;
    let t1479 = F::new(1.0) / t53 / t1477;
    let t1480 = sigma2 * t1479;
    (t1474, t1477, t1479, t1480)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 19/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk19(t52: f64, sigma0: f64, sigma1: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t53 = t52 / 2.0_f64;
    let t54 = pow_1_3(t53);
    let t55 = t54 * t54;
    let t56 = t55 * t53;
    let t59 = sigma0 + 2.0_f64 * sigma1 + sigma2;
    (t54, t55, t56, t59)
}

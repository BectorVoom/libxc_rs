//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1782/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1782(t22695: f64, t22704: f64, t22705: f64, t22863: f64, t6979: f64, t22641: f64, t3749: f64, t6978: f64, t80854: f64, t22719: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t81050 = t22704 * t22705 * t22695;
    let t81061 = t22863 * t6979;
    let t81064 = t22641 * t3749;
    let t81066 = t81064 * t80854 * t6978;
    let t81069 = t6897 * t794 * t22719;
    (t81050, t81061, t81064, t81066, t81069)
}

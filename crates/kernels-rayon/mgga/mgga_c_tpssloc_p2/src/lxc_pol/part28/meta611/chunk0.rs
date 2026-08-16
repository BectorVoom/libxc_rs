//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1923/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923(t22633: f64, t26421: f64, t3856: f64, t6976: f64, t26462: f64, t6914: f64, t22705: f64, t26414: f64, t81228: f64, t26415: f64, t81159: f64, t3851: f64) -> (f64, f64, f64, f64, f64) {
    let t90933 = t22633 * t6976 * t26421 * t3856;
    let t90956 = t6914 * t26462;
    let t90961 = t81228 * t22705 * t26414;
    let t90963 = t81159 * t26415;
    let t90968 = t22633 * t6976 * t26421 * t3851;
    (t90933, t90956, t90961, t90963, t90968)
}

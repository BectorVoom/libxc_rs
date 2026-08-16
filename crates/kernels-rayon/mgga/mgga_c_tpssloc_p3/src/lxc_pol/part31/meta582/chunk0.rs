//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1822/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1822(t22705: f64, t26422: f64, t81228: f64, t22704: f64, t26466: f64, t26461: f64, t26433: f64, t6883: f64, t22716: f64, t7741: f64, t81039: f64, t81061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90844 = t81228 * t22705 * t26422;
    let t90859 = t22704 * t22705 * t26466;
    let t90864 = t22704 * t22705 * t26461;
    let t90866 = t6883 * t26433;
    let t90868 = t22716 * t7741;
    let t90876 = 0.12793931631041761173e0_f64 * t81039;
    let t90889 = 0.12793931631041761173e0_f64 * t81061;
    (t90844, t90859, t90864, t90866, t90868, t90876, t90889)
}

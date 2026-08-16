//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1740/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1740(t193: f64, t776: f64, t111: f64, t5363: f64, t6470: f64, t19297: f64, t604: f64, t2239: f64, t5385: f64, t19449: f64, t19644: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46341 = t193 * t776;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55880 = t19297 * t604;
    let t55921 = t5385 * t2239;
    let t55943 = t19449 * t111;
    let t56422 = t19644 * t225;
    (t46341, t55353, t55388, t55880, t55921, t55943, t56422)
}

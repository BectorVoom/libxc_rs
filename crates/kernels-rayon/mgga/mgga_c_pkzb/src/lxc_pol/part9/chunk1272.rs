//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1272/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1272(t22225: f64, t22308: f64, t881: f64, t890: f64, t898: f64, t18427: f64, t18430: f64, t18433: f64, t18445: f64, t18448: f64, t18554: f64, t18555: f64, t22190: f64, t22193: f64, t22196: f64, t22199: f64, t22202: f64, t22205: f64, t22207: f64, t22209: f64, t22215: f64, t22217: f64, t22220: f64, t22222: f64) -> (f64, f64, f64) {
    let t22309 = t22225 + t22308;
    let t22313 = 0.5848223622634646207e0_f64 * t898 * t881 * t22309 * t890;
    let t22331 = 0.427258125e1_f64 * t22190 - 0.230371875e0_f64 * t22193 - 0.3560484375e1_f64 * t22196 + 0.1151859375e0_f64 * t22199 - 0.28483875e1_f64 * t22202 + 0.46074375e0_f64 * t22205 - 0.28483875e1_f64 * t22207 - 0.9494625e0_f64 * t22209 + t18554 - 0.27903555555555555556e1_f64 * t18427 + 0.11958666666666666667e1_f64 * t18430 - 0.29896666666666666667e0_f64 * t18433 + t18555 + 0.82156666666666666666e0_f64 * t18448 + 0.46074375e0_f64 * t22215 + 0.15358125e0_f64 * t22217 + 0.427258125e1_f64 * t22220 - 0.230371875e0_f64 * t22222 - 0.21908444444444444445e1_f64 * t18445;
    (t22309, t22313, t22331)
}

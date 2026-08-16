//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1270/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1270(t218: f64, t675: f64, t7984: f64, t7988: f64, t1180: f64, t5555: f64, t1878: f64, t3061: f64, t3065: f64, t22233: f64, t18427: f64, t18430: f64, t18433: f64, t18468: f64, t22230: f64, t22236: f64, t22262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22284 = t218 * t675 * t7984;
    let t22287 = t218 * t675 * t7988;
    let t22290 = t218 * t5555 * t1180;
    let t22293 = t218 * t1878 * t3061;
    let t22294 = 0.82785e0_f64 * t22293;
    let t22296 = t218 * t1878 * t3065;
    let t22297 = 0.82785e0_f64 * t22296;
    let t22302 = 4.0_f64 / 3.0_f64 * t22233;
    let t22303 = t18468 - 28.0_f64 / 9.0_f64 * t18427 + 4.0_f64 / 3.0_f64 * t18430 - t18433 / 3.0_f64 - 28.0_f64 / 27.0_f64 * t22230 + t22302 - t22236 + t22262;
    (t22284, t22287, t22290, t22293, t22294, t22296, t22297, t22303)
}

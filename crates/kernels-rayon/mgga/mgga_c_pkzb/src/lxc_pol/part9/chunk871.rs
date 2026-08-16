//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 871/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk871(t237: f64, t6297: f64, t6356: f64, t2461: f64, t955: f64, t2463: f64, t418: f64, t2411: f64, t300: f64, t2226: f64, t394: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6358 = t237 * (t6297 + t6356);
    let t6359 = t2461 * t955;
    let t6362 = 1.0_f64 / t2463 / t418;
    let t6366 = t300 * t2411;
    let t6367 = t394 * t2226;
    let t6368 = t944 * t6367;
    (t6358, t6359, t6362, t6366, t6367, t6368)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 623/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk623(t1090: f64, t7363: f64, t7362: f64, t1186: f64, t2148: f64, t50: f64, t6794: f64, t131: f64, t467: f64, t1009: f64, t461: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7364 = t7363 * t1090;
    let t7365 = t7362 * t7364;
    let t7368 = t1186 * t2148;
    let t7371 = t50 * t6794;
    let t7372 = t7371 * t131;
    let t7373 = t7372 * t467;
    let t7374 = t461 * t1009;
    let t7375 = t7374 * t1209;
    (t7364, t7365, t7368, t7371, t7372, t7373, t7375)
}

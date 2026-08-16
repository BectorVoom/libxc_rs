//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 773/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk773(t24217: f64, t24233: f64, t218: f64, t7084: f64, t798: f64, t23013: f64, t23031: f64, t2684: f64, t7101: f64, t2047: f64, t2627: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24234 = t24217 + t24233;
    let t24235 = t218 * t24234;
    let t24237 = t798 * t7084;
    let t24246 = 0.12793931631041761173e0_f64 * t23013;
    let t24250 = 0.52089578783527170489e-1_f64 * t23031;
    let t24251 = t7101 * t2684;
    let t24255 = t2627 * t2047;
    let t24256 = t24255 * t2633;
    (t24234, t24235, t24237, t24246, t24250, t24251, t24256)
}

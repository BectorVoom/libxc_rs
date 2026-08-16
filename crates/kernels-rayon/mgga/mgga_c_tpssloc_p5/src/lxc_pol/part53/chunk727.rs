//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 727/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk727(t1351: f64, t1799: f64, t120: f64, t5286: f64, t1824: f64, t3792: f64, t225: f64, t5319: f64, t5217: f64, t112: f64, t5363: f64, t111: f64, t1851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16225 = t1799 * t1351;
    let t16242 = t120 * t5286;
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    (t16225, t16242, t16306, t16311, t16439, t16460, t16521, t16524)
}

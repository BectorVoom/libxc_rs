//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1369/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1369(t11064: f64, t42332: f64, t11058: f64, t3185: f64, t42741: f64, t10481: f64, t1049: f64, t3040: f64, t3166: f64, t1014: f64, t42340: f64, t42341: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    let t43480 = t42741 * t3185;
    let t43483 = t1049 * t10481;
    let t43489 = t3166 * t3040;
    let t43503 = t42340 * t42341 * t1014;
    (t43470, t43473, t43480, t43483, t43489, t43503)
}

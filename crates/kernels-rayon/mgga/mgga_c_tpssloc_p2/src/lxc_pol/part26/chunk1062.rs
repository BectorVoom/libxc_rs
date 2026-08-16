//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1062/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1062(t1932: f64, t3493: f64, t475: f64, t1089: f64, t607: f64, t1215: f64, t1307: f64, t1388: f64, t1351: f64, t2319: f64, t576: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15022 = t1932 * t3493 * t475;
    let t15701 = t475 * t1089;
    let t15702 = t15701 * t607;
    let t15707 = t607 * t1215;
    let t15904 = t1388 * t1307;
    let t16312 = t1351 * t1307;
    let t16535 = t576 * t2319;
    let t20173 = t576 * t671;
    (t15022, t15702, t15707, t15904, t16312, t16535, t20173)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 840/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk840(t225: f64, t6401: f64, t6402: f64, t3843: f64, t6330: f64, t1347: f64, t6347: f64, t1819: f64, t1821: f64, t546: f64, t548: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t6404 = (t6401 + t6402) * t225;
    let t6408 = t3843 * t6330;
    let t6411 = t1347 * t6347;
    let t6414 = 6.0_f64 * t1819 * t1821 - 12.0_f64 * t546 * t6408 + 3.0_f64 * t546 * t6411 - t548 * t6404;
    let t6415 = t6414 * t550;
    (t6404, t6408, t6411, t6414, t6415)
}

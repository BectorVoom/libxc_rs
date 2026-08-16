//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 837/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk837(t1807: f64, t1834: f64, t119: f64, t6330: f64, t210: f64, t6347: f64, t225: f64, t6361: f64, t554: f64, t1824: f64, t3792: f64, t1343: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6364 = t1807 * t1834;
    let t6370 = t119 * t6330;
    let t6371 = t210 * t6370;
    let t6374 = t119 * t6347;
    let t6375 = t210 * t6374;
    let t6378 = t6361 * t225;
    let t6379 = t6378 * t554;
    let t6387 = t1824 * t1824;
    let t6388 = t6387 * t3792;
    let t6390 = t1343 * t820 * t6388;
    (t6364, t6370, t6371, t6374, t6375, t6378, t6379, t6387, t6388, t6390)
}

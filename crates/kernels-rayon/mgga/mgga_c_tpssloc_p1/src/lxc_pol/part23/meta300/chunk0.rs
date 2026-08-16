//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1029/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1029(t21510: f64, t4588: f64, t4582: f64, t10970: f64, t21130: f64, t248: f64, t1616: f64, t5681: f64, t3071: f64, t1539: f64, t5873: f64, t10403: f64, t1041: f64, t13966: f64, t13995: f64, t17621: f64, t17625: f64, t17656: f64, t17660: f64, t17662: f64, t17668: f64, t21503: f64, t3039: f64, t3070: f64, t5909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21511 = t4588 * t21510;
    let t21512 = t4582 * t21511;
    let t21516 = t248 * t10970 * t21130;
    let t21519 = t5681 * t1616;
    let t21520 = t3071 * t21519;
    let t21525 = t5873 * t1539;
    let t21526 = t3071 * t21525;
    let t21529 = t17621 / 216.0_f64 - t13966 / 4608.0_f64 - t17625 / 144.0_f64 - t3039 * t21503 / 1024.0_f64 - t17656 / 1536.0_f64 + t17660 / 2304.0_f64 + t17662 / 768.0_f64 + t17668 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t1041 * t21512 + 5.0_f64 / 5184.0_f64 * t1041 * t21516 - t3070 * t21520 / 768.0_f64 + t13995 * t5909 / 768.0_f64 + t10403 * t21526 / 768.0_f64;
    (t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529)
}

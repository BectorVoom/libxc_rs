//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1053/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1053(t12437: f64, t1378: f64, t12237: f64, t562: f64, t12434: f64, t539: f64, t225: f64, t3755: f64, t12016: f64, t12023: f64, t12027: f64, t12030: f64, t12033: f64, t12036: f64, t1375: f64, t1386: f64, t3758: f64, t3882: f64, t3889: f64, t3912: f64, t568: f64) -> (f64, f64, f64, f64, f64) {
    let t12438 = t1378 * t12437;
    let t12440 = t12237 * t562;
    let t12442 = t539 * t12434;
    let t12444 = t3755 * t225;
    let t12451 = 3.0_f64 * t12016 * t568 - 6.0_f64 * t12023 * t1375 + 6.0_f64 * t12027 * t1375 - 3.0_f64 * t12030 * t1386 - 3.0_f64 * t12033 * t1386 + 3.0_f64 * t12036 * t568 - t12438 * t1375 + t12440 * t568 + t12442 * t568 - 6.0_f64 * t12444 * t1386 + 6.0_f64 * t3758 * t3889 - 3.0_f64 * t3758 * t3912 + 6.0_f64 * t3882 * t3889 - 3.0_f64 * t3882 * t3912;
    (t12438, t12440, t12442, t12444, t12451)
}

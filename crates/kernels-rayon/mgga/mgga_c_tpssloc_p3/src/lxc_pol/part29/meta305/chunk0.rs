//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1347/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1347(t2250: f64, t2989: f64, t2775: f64, t343: f64, t2244: f64, t2987: f64, t3014: f64, t2262: f64, t972: f64, t2960: f64, t2971: f64, t2970: f64, t2995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10245 = t2989 * t2250;
    let t10254 = t343 * t2775;
    let t10255 = t10254 * t2244;
    let t10259 = t2987 * t3014;
    let t10263 = t2262 * t972;
    let t10267 = t2960 * t2971;
    let t10273 = t2970 * t2995;
    (t10245, t10254, t10255, t10259, t10263, t10267, t10273)
}

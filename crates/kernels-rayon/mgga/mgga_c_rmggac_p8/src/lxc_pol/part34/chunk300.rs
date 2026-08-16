//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 300/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk300(t2392: f64, t262: f64, t2079: f64, t2376: f64, t305: f64, t2379: f64, t326: f64, t118: f64, t2292: f64, t2367: f64, t338: f64, t2066: f64, t2087: f64, t2382: f64, t2384: f64, t2386: f64, t2388: f64, t2390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2393 = t262 * t2392;
    let t2394 = t2079 * t2393;
    let t2396 = t305 * t2376;
    let t2398 = t326 * t2379;
    let t2400 = t118 * t2292;
    let t2402 = t338 * t2367;
    let t2403 = t118 * t2402;
    let t2405 = 0.2993560425465952141e-1_f64 * t2382 - 0.44903406381989282115e-1_f64 * t2384 - 0.14967802127329760705e-1_f64 * t2386 - t2066 - 0.10227998120342003148e-1_f64 * t2388 + 0.13637330827122670864e-1_f64 * t2390 + 0.34093327067806677161e-2_f64 * t2394 + t2087 + 0.59871208509319042821e-1_f64 * t2396 - 0.59871208509319042821e-1_f64 * t2398 - 0.39914139006212695214e-1_f64 * t2400 + 0.19957069503106347607e-1_f64 * t2403;
    (t2394, t2396, t2398, t2400, t2402, t2403, t2405)
}

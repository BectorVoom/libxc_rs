//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1979/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1979(t108379: f64, t7515: f64, t102361: f64, t102363: f64, t102364: f64, t102367: f64, t108282: f64, t1444: f64, t2097: f64, t22386: f64, t22395: f64, t25921: f64, t25924: f64, t25930: f64, t27837: f64, t27868: f64, t28850: f64, t28911: f64, t28918: f64, t30105: f64, t30227: f64, t30296: f64, t30308: f64, t7292: f64, t7295: f64, t7296: f64, t7511: f64, t75188: f64, t7523: f64, t96392: f64, t97933: f64) -> f64 {
    let t109609 = t108379 * t7515;
    let t109628 = -0.17347256376410398924e1_f64 * t27868 * t28911 * t75188 - 0.17347256376410398924e1_f64 * t97933 * t28918 + t102361 + t102363 - 0.17347256376410398924e1_f64 * t25930 * t96392 * t30105 + 0.8673628188205199462e0_f64 * t108282 * t7523 + 0.72280234901709995518e-2_f64 * t109609 + 0.26341796731742046394e1_f64 * t7511 * t22395 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t30308 * t1444 - 0.45699670022203476294e-2_f64 * t102364 + t102367 - 0.4336814094102599731e0_f64 * t7292 * t30296 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2097 * t22386 + 0.8673628188205199462e0_f64 * t27837 * t28850 - 0.8673628188205199462e0_f64 * t25921 * t30227;
    t109628
}

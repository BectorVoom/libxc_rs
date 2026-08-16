//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2110/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2110(t25904: f64, t98303: f64, t786: f64, t97961: f64, t7286: f64, t2439: f64, t7925: f64, t94391: f64, t94383: f64, t1444: f64, t213: f64, t225: f64, t25921: f64, t25924: f64, t25930: f64, t25931: f64, t25961: f64, t27837: f64, t27846: f64, t27858: f64, t27902: f64, t561: f64, t7295: f64, t94876: f64, t98099: f64, t98101: f64, t98104: f64, t98290: f64, t98299: f64) -> f64 {
    let t98305 = 0.14456046980341999104e-1_f64 * t25904 * t98303;
    let t98308 = t786 * t97961;
    let t98310 = 0.14456046980341999104e-1_f64 * t98308 * t7286;
    let t98311 = t7925 * t2439;
    let t98312 = t94391 * t98311;
    let t98314 = t94383 * t98311;
    let t98318 = 0.8673628188205199462e0_f64 * t25921 * t27846 - 0.24093411633903331839e-3_f64 * t98099 - 0.17135234354032049604e-1_f64 * t98101 - 0.45699670022203476294e-2_f64 * t94876 - 0.96373646535613327357e-2_f64 * t98104 + 0.65854491829355115987e0_f64 * t213 * t98290 * t225 * t561 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t27902 * t1444 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t98299 - t98305 + 0.8673628188205199462e0_f64 * t27837 * t25961 + t98310 + 0.22849835011101738147e-2_f64 * t98312 - 0.17135234354032049604e-2_f64 * t98314 + 0.8673628188205199462e0_f64 * t25921 * t27858;
    t98318
}

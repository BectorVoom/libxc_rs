//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1348/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1348(t66398: f64, t66411: f64, t66425: f64, t66439: f64, t2157: f64, t5831: f64, t1395: f64, t18770: f64, t10841: f64, t1378: f64, t1707: f64, t1708: f64, t17993: f64, t18000: f64, t18006: f64, t18009: f64, t1809: f64, t18784: f64, t18800: f64, t19736: f64, t19767: f64, t19769: f64, t20446: f64, t20466: f64, t20470: f64, t20482: f64, t20488: f64, t20503: f64, t226: f64, t228: f64, t2364: f64, t2407: f64, t253: f64, t44584: f64, t44610: f64, t5571: f64, t5577: f64, t5834: f64, t61195: f64, t61222: f64, t61226: f64, t6135: f64, t6337: f64, t6342: f64, t63893: f64, t64008: f64, t64050: f64, t64198: f64, t782: f64, t818: f64, param_beta: f64) -> f64 {
    let t66441 = t66398 + t66411 + t66425 + t66439;
    let t66469 = t2157 * t5831;
    let t66480 = t18770 * t1395;
    let t66494 = 24.0_f64 * t5571 * t61195 * t6342 * t2407 - 2.0_f64 * t19736 * t18784 - t1707 * t1708 * t228 * t66441 + param_beta * t66441 * t253 - 2.0_f64 * t18006 * t18770 * t64008 + 2.0_f64 * t17993 * t20503 + 2.0_f64 * t5571 * t5577 * t20446 * t782 * t226 + 2.0_f64 * t5834 * t10841 - t64050 * t1809 + 8.0_f64 * t18006 * t20482 * t1378 * t63893 + 2.0_f64 * t19767 * t18770 * t44610 - 4.0_f64 * t61222 * t20466 - 4.0_f64 * t19767 * t66469 * t19769 - 4.0_f64 * t19767 * t20482 * t44584 - 4.0_f64 * t18006 * t18770 * t64198 - t6135 * t18800 + 12.0_f64 * t61226 * t66480 * t18009 - 12.0_f64 * t5571 * t18000 * t20470 * t818 + t5571 * t5577 * t6337 * t2364 * t226 + 2.0_f64 * t17993 * t20488;
    t66494
}

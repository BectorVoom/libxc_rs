//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3005/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005(t13995: f64, t14501: f64, t1020: f64, t1021: f64, t10214: f64, t10403: f64, t10408: f64, t1041: f64, t14164: f64, t14211: f64, t1539: f64, t17701: f64, t17732: f64, t18014: f64, t248: f64, t2979: f64, t3040: f64, t3070: f64, t3071: f64, t3120: f64, t360: f64, t42388: f64, t42546: f64, t42861: f64, t43343: f64, t4338: f64, t4582: f64, t4650: f64, t48612: f64, t50337: f64, t5875: f64, t59706: f64, t59711: f64, t59719: f64, t61910: f64, t62757: f64, t62766: f64, t62778: f64, t973: f64) -> f64 {
    let t62780 = t13995 * t14501;
    let t62803 = t1041 * t4582 * t14164 * t61910 / 768.0_f64 + t1020 * t248 * t1021 * t62757 * t360 / 3072.0_f64 + t43343 * t5875 / 1536.0_f64 + 7.0_f64 / 972.0_f64 * t62766 + t973 * t2979 * t59719 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t973 * t10214 * t59706 + 35.0_f64 / 972.0_f64 * t973 * t42861 * t59711 - t62778 / 384.0_f64 + t62780 / 1728.0_f64 + 19.0_f64 / 1296.0_f64 * t50337 + t10403 * t3071 * t17732 * t18014 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t3070 * t10408 * t4650 * t4338 + t10403 * t3071 * t14211 * t1539 * t3120 / 1152.0_f64 + t42388 * t3071 * t48612 * t1539 * t3040 / 384.0_f64 - t42546 * t17701 / 2304.0_f64;
    t62803
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3243/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243(t61496: f64, t61517: f64, t10871: f64, t5977: f64, t14931: f64, t18477: f64, t51123: f64, t10811: f64, t18471: f64, t18451: f64, t124: f64, t14772: f64, t14786: f64, t14791: f64, t14802: f64, t14894: f64, t1559: f64, t18632: f64, t2745: f64, t4362: f64, t50312: f64, t50325: f64, t50328: f64, t50347: f64, t51014: f64, t799: f64, t800: f64) -> (f64, f64, f64) {
    let t61519 = t61496 / 2.0_f64 + t61517 / 2.0_f64;
    let t61532 = t5977 * t10871;
    let t61538 = t14931 * t51123 * t18477;
    let t61540 = t10811 * t18471;
    let t61542 = t10811 * t18451;
    let t61544 = -0.28582678745379824648e-3_f64 * t50312 + 0.10164000561857065645e-3_f64 * t50325 + 0.20007875121765877254e-2_f64 * t50328 + 0.32012600194825403606e-1_f64 * t50347 - t799 * t800 * t124 * t61519 / 48.0_f64 - 0.68598428988911579156e-2_f64 * t4362 * t14791 * t18632 * t14786 + 0.51448821741683684367e-1_f64 * t2745 * t51014 * t1559 * t14772 + 0.10289764348336736874e-1_f64 * t14894 * t14791 * t61532 * t14802 - 0.4065600224742826258e-3_f64 * t61538 + 0.40015750243531754508e-1_f64 * t61540 - 0.80031500487063509015e-2_f64 * t61542;
    (t61519, t61532, t61544)
}

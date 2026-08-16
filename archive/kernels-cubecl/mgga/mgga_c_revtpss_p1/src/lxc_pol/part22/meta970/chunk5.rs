//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3243/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243<F: Float>(t61496: F, t61517: F, t10871: F, t5977: F, t14931: F, t18477: F, t51123: F, t10811: F, t18471: F, t18451: F, t124: F, t14772: F, t14786: F, t14791: F, t14802: F, t14894: F, t1559: F, t18632: F, t2745: F, t4362: F, t50312: F, t50325: F, t50328: F, t50347: F, t51014: F, t799: F, t800: F) -> (F, F, F) {
    let t61519 = t61496 / F::cast_from(2.0_f64) + t61517 / F::cast_from(2.0_f64);
    let t61532 = t5977 * t10871;
    let t61538 = t14931 * t51123 * t18477;
    let t61540 = t10811 * t18471;
    let t61542 = t10811 * t18451;
    let t61544 = -F::cast_from(0.28582678745379824648e-3_f64) * t50312 + F::cast_from(0.10164000561857065645e-3_f64) * t50325 + F::cast_from(0.20007875121765877254e-2_f64) * t50328 + F::cast_from(0.32012600194825403606e-1_f64) * t50347 - t799 * t800 * t124 * t61519 / F::cast_from(48.0_f64) - F::cast_from(0.68598428988911579156e-2_f64) * t4362 * t14791 * t18632 * t14786 + F::cast_from(0.51448821741683684367e-1_f64) * t2745 * t51014 * t1559 * t14772 + F::cast_from(0.10289764348336736874e-1_f64) * t14894 * t14791 * t61532 * t14802 - F::cast_from(0.4065600224742826258e-3_f64) * t61538 + F::cast_from(0.40015750243531754508e-1_f64) * t61540 - F::cast_from(0.80031500487063509015e-2_f64) * t61542;
    (t61519, t61532, t61544)
}

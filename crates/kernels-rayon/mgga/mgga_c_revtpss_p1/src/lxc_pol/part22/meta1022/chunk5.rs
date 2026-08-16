//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3566/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3566(t20112: f64, t342: f64, t15669: f64, t1678: f64, t1076: f64, t1079: f64, t1097: f64, t11121: f64, t16152: f64, t16255: f64, t16275: f64, t16333: f64, t16597: f64, t1696: f64, t19429: f64, t20219: f64, t3058: f64, t3060: f64, t3063: f64, t3270: f64, t3325: f64, t33754: f64, t4752: f64, t4941: f64, t4947: f64, t53015: f64, t53034: f64, t53174: f64, t53180: f64, t6244: f64, t6392: f64) -> f64 {
    let t68138 = t342 * t20112;
    let t68144 = t15669 * t1678;
    let t68163 = 0.52683593463484092788e1_f64 * t4752 * t16255 + 0.52683593463484092788e1_f64 * t16333 * t4947 - 0.79025390195226139182e1_f64 * t53015 * t16275 - 0.13170898365871023197e1_f64 * t68138 * t1097 - 0.15805078039045227836e2_f64 * t53174 * t33754 * t16152 + 0.26341796731742046394e1_f64 * t68144 * t3060 - 0.13170898365871023197e1_f64 * t3058 * t1079 * t6244 * t3325 - 0.52683593463484092788e1_f64 * t53034 * t19429 - 0.13170898365871023197e1_f64 * t53180 * t1696 - 0.39512695097613069591e1_f64 * t1076 * t11121 * t6392 * t3270 + 0.13170898365871023197e1_f64 * t3063 * t20219 + 0.26341796731742046394e1_f64 * t16597 * t4941;
    t68163
}

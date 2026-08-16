//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2944/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944(t11670: f64, t15904: f64, t12167: f64, t11922: f64, t16081: f64, t16083: f64, t11675: f64, t15682: f64, t11711: f64, t15618: f64, t11667: f64, t11696: f64, t11703: f64, t11705: f64, t11866: f64, t15697: f64, t15917: f64, t15957: f64, t16022: f64, t16045: f64, t16084: f64, t19741: f64, t3091: f64, t3092: f64, t42397: f64, t43066: f64, t4781: f64) -> (f64, f64) {
    let t53552 = t11670 * t15904;
    let t53553 = t12167 * t53552;
    let t53557 = t16081 * t11922 * t16083;
    let t53559 = t11675 * t15682;
    let t53567 = t15618 * t11711;
    let t53581 = 0.45732285992607719436e-2_f64 * t43066 * t15697 - 0.20579528696673473747e-1_f64 * t53553 * t16084 + 0.25724410870841842184e-2_f64 * t53557 + 0.57165357490759649295e-3_f64 * t53559 - 0.42874018118069736972e-3_f64 * t19741 * t11667 - 0.14291339372689912324e-2_f64 * t3091 * t11703 * t4781 * t42397 + 0.57165357490759649295e-3_f64 * t53567 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t15957 * t11696 + 0.7145669686344956162e-3_f64 * t3091 * t11703 * t15957 * t11705 - 0.64311027177104605458e-3_f64 * t11866 * t16045 - 0.64311027177104605458e-3_f64 * t15917 * t16022;
    (t53552, t53581)
}

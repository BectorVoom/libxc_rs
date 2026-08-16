//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2944/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2944<F: Float>(t11670: F, t15904: F, t12167: F, t11922: F, t16081: F, t16083: F, t11675: F, t15682: F, t11711: F, t15618: F, t11667: F, t11696: F, t11703: F, t11705: F, t11866: F, t15697: F, t15917: F, t15957: F, t16022: F, t16045: F, t16084: F, t19741: F, t3091: F, t3092: F, t42397: F, t43066: F, t4781: F) -> (F, F) {
    let t53552 = t11670 * t15904;
    let t53553 = t12167 * t53552;
    let t53557 = t16081 * t11922 * t16083;
    let t53559 = t11675 * t15682;
    let t53567 = t15618 * t11711;
    let t53581 = F::cast_from(0.45732285992607719436e-2_f64) * t43066 * t15697 - F::cast_from(0.20579528696673473747e-1_f64) * t53553 * t16084 + F::cast_from(0.25724410870841842184e-2_f64) * t53557 + F::cast_from(0.57165357490759649295e-3_f64) * t53559 - F::cast_from(0.42874018118069736972e-3_f64) * t19741 * t11667 - F::cast_from(0.14291339372689912324e-2_f64) * t3091 * t11703 * t4781 * t42397 + F::cast_from(0.57165357490759649295e-3_f64) * t53567 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t15957 * t11696 + F::cast_from(0.7145669686344956162e-3_f64) * t3091 * t11703 * t15957 * t11705 - F::cast_from(0.64311027177104605458e-3_f64) * t11866 * t16045 - F::cast_from(0.64311027177104605458e-3_f64) * t15917 * t16022;
    (t53552, t53581)
}

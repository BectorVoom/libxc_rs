//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1422/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1422(t34949: f64, t34954: f64, t34962: f64, t37134: f64, t37135: f64, t37136: f64, t37138: f64, t37140: f64, t37141: f64, t37142: f64, t37144: f64, t35034: f64, t35041: f64, t37172: f64, t37174: f64, t37175: f64, t37177: f64, t37178: f64, t37179: f64, t37180: f64, t37181: f64, t37182: f64) -> (f64, f64) {
    let t38637 = t37134 - t37135 + t37136 - 0.18115908419564701086e-6_f64 * t34949 - t37138 - 0.98380106748709416171e-8_f64 * t34954 - t37140 - t37141 - t37142 - 0.18115908419564701086e-6_f64 * t34962 - t37144;
    let t38647 = -t37172 - 0.4419852458519115466e-7_f64 * t35034 - t37174 - t37175 - 0.57970906942607043475e-5_f64 * t35041 - t37177 - t37178 - t37179 + t37180 + t37181 - t37182;
    (t38637, t38647)
}

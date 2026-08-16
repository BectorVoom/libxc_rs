//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3015/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3015(t11773: f64, t15925: f64, t11783: f64, t4845: f64, t15745: f64, t3215: f64, t11792: f64, t15749: f64, t3224: f64, t11776: f64, t11866: f64, t15922: f64, t16186: f64, t1665: f64, t3169: f64, t42290: f64, t42355: f64, t43038: f64, t43238: f64, t4907: f64) -> f64 {
    let t55141 = t15925 * t11773;
    let t55148 = t11783 * t4845;
    let t55150 = t15745 * t3215;
    let t55152 = t11792 * t4845;
    let t55154 = t3224 * t15749;
    let t55155 = 0.14291339372689912324e-3_f64 * t55154;
    let t55163 = -0.85748036236139473944e-3_f64 * t55141 * t11776 + 0.53100265402527852012e-1_f64 * t42355 * t1665 + 0.34299214494455789577e-2_f64 * t42290 * t1665 - 0.42874018118069736972e-3_f64 * t55148 + 0.45732285992607719436e-2_f64 * t55150 + 0.45732285992607719436e-2_f64 * t55152 + t55155 - 0.34299214494455789577e-2_f64 * t3169 * t16186 - 0.10162730220579493208e-2_f64 * t43238 - 0.64311027177104605458e-3_f64 * t43038 * t4907 - 0.12862205435420921092e-2_f64 * t11866 * t15922;
    t55163
}

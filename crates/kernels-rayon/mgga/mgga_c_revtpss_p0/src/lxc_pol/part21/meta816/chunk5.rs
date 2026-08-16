//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3000/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3000(t15816: f64, t3168: f64, t10326: f64, t10356: f64, t1047: f64, t11144: f64, t11675: f64, t15599: f64, t15601: f64, t15622: f64, t1592: f64, t3091: f64, t3092: f64, t3094: f64, t3095: f64, t357: f64, t42410: f64, t42610: f64, t42965: f64, t42996: f64, t43003: f64, t43297: f64, t4583: f64, t4781: f64, t54026: f64) -> f64 {
    let t54739 = t15816 * t3168;
    let t54770 = -0.57165357490759649295e-3_f64 * t42965 - 0.13719685797782315831e-1_f64 * t43297 * t15622 - 0.68598428988911579154e-2_f64 * t54739 * t1047 + 0.19055119163586549765e-3_f64 * t42996 + 0.14291339372689912324e-3_f64 * t43003 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t54026 * t3095 + 0.14291339372689912324e-3_f64 * t3091 * t3092 * t4781 * t3094 * t10326 + 0.63517063878621832552e-3_f64 * t3091 * t42410 * t4781 * t357 * t11144 * t10356 + 0.42874018118069736972e-3_f64 * t11675 * t15601 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t4583 * t15599 + 0.14291339372689912324e-3_f64 * t3091 * t3092 * t1592 * t42610 * t357;
    t54770
}

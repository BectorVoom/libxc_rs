//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 616/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk616(t15502: f64, t515: f64, t7231: f64, t3351: f64, t15255: f64, t15259: f64, t15263: f64, t15267: f64, t15269: f64, t15273: f64, t664: f64, t9530: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15503 = t515 * t15502;
    let t15504 = t7231 * t15503;
    let t15505 = t3351 * t15504;
    let t15506 = 0.42564599893297839398e-5_f64 * t15505;
    let t15510 = 0.85129199786595678799e-5_f64 * t15255;
    let t15511 = 0.2553875993597870364e-4_f64 * t15259;
    let t15512 = 0.2553875993597870364e-4_f64 * t15263;
    let t15513 = 0.1702583995731913576e-4_f64 * t15267;
    let t15514 = 0.85129199786595678799e-5_f64 * t15269;
    let t15515 = 0.31062809106223861415e-2_f64 * t15273;
    let t15516 = t9530 * t664;
    (t15504, t15506, t15510, t15511, t15512, t15513, t15514, t15515, t15516)
}

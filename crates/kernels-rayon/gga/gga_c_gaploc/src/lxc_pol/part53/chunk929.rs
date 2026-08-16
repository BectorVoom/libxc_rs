//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 929/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk929(t41290: f64, t41293: f64, t41295: f64, t41299: f64, t41312: f64, t41316: f64, t13016: f64, t8478: f64, t10867: f64, t1423: f64, t3247: f64, t41330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43886 = 0.29792074959875355558e-1_f64 * t41290;
    let t43887 = 0.59584149919750711116e-1_f64 * t41293;
    let t43888 = 0.63904876589867916127e-1_f64 * t41295;
    let t43889 = 0.63904876589867916127e-1_f64 * t41299;
    let t43892 = 0.63904876589867916127e-1_f64 * t41312;
    let t43893 = 0.63904876589867916127e-1_f64 * t41316;
    let t43895 = 0.10725146985555128001e1_f64 * t8478 * t13016;
    let t43907 = t10867 * t1423 * t3247;
    let t43908 = 0.17875244975925213335e0_f64 * t43907;
    let t43909 = 0.11502877786176224903e1_f64 * t41330;
    (t43886, t43887, t43888, t43889, t43892, t43893, t43895, t43908, t43909)
}

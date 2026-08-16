//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 507/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk507(t2953: f64, t2954: f64, t1005: f64, t1599: f64, t1603: f64, t2937: f64, t2913: f64, t2916: f64, t2926: f64, t2930: f64, t2934: f64, t2939: f64, t2943: f64, t2946: f64, t2949: f64) -> (f64, f64, f64) {
    let t2955 = t2953 * t2954;
    let t2957 = t1005 * t1599;
    let t2958 = t2937 * t1603;
    let t2959 = t2957 * t2958;
    let t2961 = -0.30368356656884499037e-4_f64 * t2913 - 0.10122785552294833012e-4_f64 * t2916 + 0.14762395597096631476e-5_f64 * t2926 - 0.30368356656884499037e-4_f64 * t2930 - 0.21724560703384400956e-4_f64 * t2934 + 0.21724560703384400956e-4_f64 * t2939 + 0.21724560703384400956e-5_f64 * t2943 - 0.386262689306174649e-5_f64 * t2946 - 0.21724560703384400956e-4_f64 * t2949 - 0.63363302051537836122e-5_f64 * t2955 + 0.21724560703384400956e-4_f64 * t2959;
    (t2957, t2958, t2961)
}

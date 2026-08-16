//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 727/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk727(t8545: f64, t996: f64, t2912: f64, t4538: f64, t2929: f64, t1599: f64, t2932: f64, t2958: f64, t8522: f64, t8526: f64, t8529: f64, t8532: f64, t8536: f64, t8539: f64, t8541: f64, t8543: f64) -> f64 {
    let t8546 = t996 * t8545;
    let t8547 = t8546 * t2912;
    let t8549 = t996 * t4538;
    let t8550 = t8549 * t2929;
    let t8552 = t2932 * t1599;
    let t8553 = t8552 * t2958;
    let t8555 = 0.30368356656884499037e-4_f64 * t8522 + 0.20245571104589666024e-4_f64 * t8526 + 0.20245571104589666024e-4_f64 * t8529 - 0.30368356656884499037e-4_f64 * t8532 - 0.98415970647310876506e-6_f64 * t8536 - 0.30368356656884499037e-4_f64 * t8539 + 0.26319242435966565832e-3_f64 * t8541 + 0.26319242435966565832e-3_f64 * t8543 - 0.60736713313768998074e-4_f64 * t8547 - 0.60736713313768998074e-4_f64 * t8550 + 0.43449121406768801912e-4_f64 * t8553;
    t8555
}

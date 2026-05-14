//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 678/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk678<F: Float>(t2929: F, t2941: F, t1845: F, t515: F, t996: F, t2912: F, t4538: F, t1599: F, t2932: F, t2958: F, t8522: F, t8526: F, t8529: F, t8532: F, t8536: F, t8539: F, t8541: F) -> (F,) {
    let t8543 = t2941 * t2929;
    let t8545 = t1845 * t515;
    let t8546 = t996 * t8545;
    let t8547 = t8546 * t2912;
    let t8549 = t996 * t4538;
    let t8550 = t8549 * t2929;
    let t8552 = t2932 * t1599;
    let t8553 = t8552 * t2958;
    let t8555 = 0.30368356656884499037e-4 * t8522 + 0.20245571104589666024e-4 * t8526 + 0.20245571104589666024e-4 * t8529 - 0.30368356656884499037e-4 * t8532 - 0.98415970647310876506e-6 * t8536 - 0.30368356656884499037e-4 * t8539 + 0.26319242435966565832e-3 * t8541 + 0.26319242435966565832e-3 * t8543 - 0.60736713313768998074e-4 * t8547 - 0.60736713313768998074e-4 * t8550 + 0.43449121406768801912e-4 * t8553;
    (t8555,)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 906/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk906<F: Float>(t43403: F, t15499: F, t28640: F, t3487: F, t9806: F, t40966: F, t2963: F, t3295: F, t9796: F, t40969: F, t1029: F, t9829: F) -> (F, F, F, F, F, F) {
    let t43404 = F::new(0.10352590007558602413e2) * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = F::new(0.46011511144704899612e1) * t43407;
    let t43409 = F::new(0.11502877786176224903e1) * t40966;
    let t43412 = t9796 * t2963 * t3295;
    let t43414 = F::new(0.38342925953920749676e1) * t40969;
    let t43416 = t9796 * t1029 * t9829;
    (t43404, t43408, t43409, t43412, t43414, t43416)
}

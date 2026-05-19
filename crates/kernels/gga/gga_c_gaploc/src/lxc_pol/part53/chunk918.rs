//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 918/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk918<F: Float>(t43199: F, t4820: F, t7513: F, t2028: F, t3038: F, t787: F, t9641: F, t33565: F, t7372: F, t33294: F, t9810: F, t43107: F, t701: F) -> (F, F, F, F, F) {
    let t43670 = F::cast_from(0.79445533226334281487e-1_f64) * t7513 * t4820 * t43199;
    let t43674 = F::cast_from(0.39722766613167140743e-1_f64) * t787 * t9641 * t3038 * t2028;
    let t43679 = t33565 * t7372;
    let t43680 = F::cast_from(0.29792074959875355558e-1_f64) * t43679;
    let t43681 = t33294 * t9810;
    let t43682 = F::cast_from(0.3575048995185042667e0_f64) * t43681;
    let t43683 = t43107 * t701;
    (t43670, t43674, t43680, t43682, t43683)
}

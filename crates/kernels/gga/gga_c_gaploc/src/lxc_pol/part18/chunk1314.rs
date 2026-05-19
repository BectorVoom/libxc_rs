//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1314/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1314<F: Float>(t10938: F, t2021: F, t23310: F, t25177: F, t959: F, t10847: F, t22693: F, t7572: F, t24554: F, t1: F, t33137: F, t20671: F, t22538: F, t24549: F) -> (F, F, F, F, F, F) {
    let t33565 = t2021 * t10938;
    let t33567 = F::cast_from(0.79445533226334281486e-1_f64) * t33565 * t23310;
    let t33568 = t25177 * t959;
    let t33569 = F::cast_from(0.29792074959875355558e-1_f64) * t33568;
    let t33572 = F::cast_from(0.18404604457881959845e2_f64) * t7572 * t22693 * t10847;
    let t33573 = t24554 * t959;
    let t33574 = F::cast_from(0.14896037479937677779e-1_f64) * t33573;
    let t33575 = t33137 * t1;
    let t33576 = t2021 * t33575;
    let t33580 = t22538 * t20671 * t24549;
    (t33567, t33569, t33572, t33574, t33576, t33580)
}

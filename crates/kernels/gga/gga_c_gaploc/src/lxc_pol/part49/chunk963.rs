//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 963/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk963<F: Float>(t2536: F, t3720: F, t2009: F, t2021: F, t47294: F, t7572: F, t7573: F, t12252: F, t2628: F, t43924: F, t43926: F, t43928: F, t43931: F, t43935: F, t43938: F, t43941: F, t43944: F, t47442: F) -> (F,) {
    let t47443 = t2536 * t3720;
    let t47445 = t2021 * t47443 * t2009;
    let t47448 = t7572 * t7573 * t47294;
    let t47450 = t12252 * t2628;
    let t47452 = t43924 - t43926 - t43928 - t43931 - t43935 - t43938 - 0.25025342966295298669e1 * t43941 - 0.92023022289409799224e1 * t43944 - t47442 - 0.35750489951850426669e0 * t47445 + 0.69017266717057349418e1 * t47448 - 0.29792074959875355558e-1 * t47450;
    (t47452,)
}

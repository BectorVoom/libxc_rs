//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1144/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1144<F: Float>(t33273: F, t7967: F, t961: F, t11853: F, t2578: F, t7199: F, t12744: F, t7418: F, t9709: F, t126: F, t190: F, t3044: F) -> (F, F, F, F) {
    let t33275 = t7967 * t33273 * t961;
    let t33278 = t2578 * t7199 * t11853;
    let t33284 = t9709 * t12744 * t7418;
    let t33287 = t126 * t190 * t3044;
    (t33275, t33278, t33284, t33287)
}

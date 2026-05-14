//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 682/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk682<F: Float>(t3516: F, t594: F, t107: F, t544: F, t1359: F, t3529: F, t11271: F, t524: F, t11218: F, t555: F, t188: F, t12380: F, t455: F, t145: F, t459: F, t12385: F, t2281: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37975 = t594 * t3516;
    let t37977 = t544 * t37975 * t107;
    let t38019 = t1359 * t3516;
    let t38051 = t1359 * t3529;
    let t38181 = t524 * t11271;
    let t38184 = t555 * t11218;
    let t38185 = t188 * t38184;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    (t37975, t37977, t38019, t38051, t38181, t38184, t38185, t39622, t39624, t39626)
}

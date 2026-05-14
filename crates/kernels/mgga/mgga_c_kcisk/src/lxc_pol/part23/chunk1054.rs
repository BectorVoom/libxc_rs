//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1054/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1054<F: Float>(t19119: F, t6279: F, t1337: F, t1402: F, t5676: F, t19127: F, t6287: F, t14405: F, t14409: F, t14434: F, t18081: F, t19273: F, t19358: F, t19381: F, t19390: F, t19394: F, t19435: F, t19440: F, t21104: F, t21110: F, t21114: F, t3077: F, t4253: F, t5933: F, t6256: F, t6267: F, t6278: F) -> (F,) {
    let t21117 = t6279 * t19119;
    let t21120 = t1337 * t1402;
    let t21121 = t21120 * t5676;
    let t21136 = t6287 * t19127;
    let t21139 = 0.1857375e-1 * t4253 * t19435 - 0.10612444444444444444e0 * t3077 * t21104 + 0.24765e-1 * t6267 * t19358 - 0.26531111111111111111e-1 * t14405 - t14409 - 0.44218518518518518518e-1 * t6278 * t21110 - 0.11791604938271604938e0 * t6278 * t21114 - 0.17687407407407407407e0 * t18081 * t21117 + 0.10612444444444444444e0 * t6278 * t21121 - 0.1857375e-1 * t4253 * t19440 + 0.9286875e-2 * t6256 * t19390 + 0.46434375e-2 * t6256 * t19394 - 0.46434375e-2 * t6256 * t19273 - 0.1857375e-1 * t14434 * t5933 + 0.123825e-1 * t6267 * t19381 - 0.15918666666666666667e0 * t6278 * t21136;
    (t21139,)
}

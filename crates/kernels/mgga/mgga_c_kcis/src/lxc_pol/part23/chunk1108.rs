//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1108/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1108<F: Float>(t16823: F, t28351: F, t4012: F, t39052: F, t491: F, t990: F, t1928: F, t3964: F, t1385: F, t27370: F, t3717: F, t5732: F, t52613: F, t7908: F, t8154: F, t11825: F) -> (F, F, F, F, F, F) {
    let t98286 = t28351 * t16823 * t4012;
    let t98290 = t39052 * t491 * t990;
    let t98294 = t3964 * t1928 * t990;
    let t98304 = t27370 * t3717 * t5732 * t1385;
    let t98308 = t7908 * t52613 * t8154;
    let t98310 = t11825 * t491;
    (t98286, t98290, t98294, t98304, t98308, t98310)
}

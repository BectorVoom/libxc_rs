//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1246/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1246<F: Float>(t98380: F, t27376: F, t28369: F, t27459: F, t28335: F, t28480: F, t7904: F, t27484: F, t8144: F, t28387: F, t61287: F, t1467: F, t1928: F) -> (F, F, F, F, F, F, F) {
    let t98381 = F::new(0.66327777777777777776e-2) * t98380;
    let t98383 = F::new(0.15445601851851851852e-3) * t28369 * t27376;
    let t98387 = F::new(0.15445601851851851852e-3) * t27459 * t28335;
    let t98388 = t28480 * t7904;
    let t98390 = t8144 * t27484;
    let t98392 = t28387 * t61287;
    let t98409 = t1467 * t1928;
    (t98381, t98383, t98387, t98388, t98390, t98392, t98409)
}

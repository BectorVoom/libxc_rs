//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1278/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1278<F: Float>(t1394: F, t5644: F, t94216: F, t27484: F, t8151: F, t27475: F, t303: F, t5628: F, t1014: F, t28473: F, t12147: F, t28438: F, t7908: F) -> (F, F, F, F, F, F) {
    let t98872 = t1394 * t94216 * t5644;
    let t98874 = t8151 * t27484;
    let t98883 = t303 * t27475 * t5628;
    let t98887 = t1014 * t28473;
    let t98888 = F::new(0.33163888888888888888e-2) * t98887;
    let t98903 = F::new(0.15445601851851851852e-3) * t7908 * t12147 * t28438;
    (t98872, t98874, t98883, t98887, t98888, t98903)
}

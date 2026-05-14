//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1111/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1111<F: Float>(t28513: F, t4142: F, t1464: F, t15956: F, t28503: F, t1394: F, t5667: F, t94216: F, t16773: F, t27387: F, t5780: F, t27364: F, t4153: F, t5663: F, t16836: F, t3717: F) -> (F, F, F, F, F, F) {
    let t98344 = t4142 * t28513;
    let t98347 = t1464 * t28503 * t15956;
    let t98350 = t1394 * t94216 * t5667;
    let t98353 = t5780 * t27387 * t16773;
    let t98357 = t4153 * t27364 * t5663;
    let t98359 = t16836 * t3717;
    (t98344, t98347, t98350, t98353, t98357, t98359)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1070/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1070<F: Float>(t22360: F, t7303: F, t5289: F, t23733: F, t7568: F, t7302: F, t22952: F, t747: F, t746: F, t1948: F, t2576: F, t7413: F, t24464: F, t24467: F, t24469: F, t24471: F, t24474: F, t24476: F, t24478: F) -> (F, F, F, F, F, F, F, F) {
    let t24480 = t7303 * t22360;
    let t24481 = t5289 * t24480;
    let t24483 = t7568 * t23733;
    let t24484 = t7302 * t24483;
    let t24486 = t747 * t22952;
    let t24487 = t746 * t24486;
    let t24488 = t1948 * t24487;
    let t24490 = t2576 * t7413;
    let t24492 = 11.0 / 27.0 * t24464 - t24467 / 24.0 - t24469 / 128.0 + 2.0 / 9.0 * t24471 + t24474 / 256.0 - t24476 / 8.0 - t24478 / 24.0 + t24481 / 72.0 + t24484 / 54.0 + t24488 / 256.0 + t24490 / 18.0;
    (t24480, t24481, t24483, t24484, t24487, t24488, t24490, t24492)
}

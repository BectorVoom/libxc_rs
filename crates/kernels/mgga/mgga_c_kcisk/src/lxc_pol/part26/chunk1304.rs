//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1304/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1304<F: Float>(t114618: F, t1411: F, t33508: F, t32000: F, t34700: F, t3759: F, t1308: F, t2158: F, t6221: F, t33373: F, t33593: F, t1440: F, t32045: F, t8255: F, t33384: F, t394: F, t8259: F) -> (F, F, F, F, F, F, F) {
    let t118712 = t1411 * t114618 * t33508;
    let t118715 = t3759 * t32000 * t34700;
    let t118718 = t6221 * t2158 * t1308;
    let t118721 = t33373 * t33593;
    let t118727 = t1411 * t32045 * t8255 * t1440;
    let t118729 = t33384 * t33593;
    let t118731 = t8259 * t394;
    (t118712, t118715, t118718, t118721, t118727, t118729, t118731)
}

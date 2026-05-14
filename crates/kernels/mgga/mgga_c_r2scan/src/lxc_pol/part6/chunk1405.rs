//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1405/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1405<F: Float>(t1859: F, t1862: F, t7006: F, t5322: F, t7657: F, t5326: F, t22140: F, t2743: F, t22181: F, t410: F, t7705: F, t21702: F, t21705: F, t21709: F, t22183: F, t22186: F, t22187: F, t22191: F, t22194: F) -> (F,) {
    let t26580 = t1859 * t7006 * t1862;
    let t26582 = t7657 * t5322;
    let t26584 = t7657 * t5326;
    let t26585 = 0.4051561992e0 * t26584;
    let t26586 = t2743 * t22140;
    let t26588 = 36.0 * t22181;
    let t26589 = t410 * t7705;
    let t26590 = 24.0 * t26589;
    let t26593 = 0.4051561992e0 * t26580 + 0.8103123984e0 * t26582 + t26585 + 0.4051561992e0 * t26586 + t21702 + t26588 - t21705 - t21709 + t26590 + 0.17544670867903938621e1 * t22183 - t22186 - 0.36914467889579063968e5 * t22187 - t22191 - t22194;
    (t26593,)
}

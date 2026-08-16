//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1312/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1312<F: Float>(t7176: F, t743: F, t7183: F, t733: F, t7167: F, t738: F, t7170: F, t7173: F, t7161: F, t1330: F, t21125: F, t21130: F, t3883: F) -> (F, F, F, F, F, F, F, F) {
    let t21721 = t743 * t7176;
    let t21723 = t733 * t7183;
    let t21725 = t738 * t7167;
    let t21727 = t738 * t7170;
    let t21729 = t743 * t7173;
    let t21731 = t733 * t7161;
    let t21734 = t1330 * t21125;
    let t21737 = t3883 * t21130;
    (t21721, t21723, t21725, t21727, t21729, t21731, t21734, t21737)
}

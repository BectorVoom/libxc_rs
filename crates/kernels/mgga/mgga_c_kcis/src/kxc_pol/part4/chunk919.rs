//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 919/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk919<F: Float>(t1416: F, t3820: F, t1317: F, t3838: F, t11407: F, t1098: F, t3843: F, t4277: F, t3728: F, t3739: F, t1466: F, t4108: F, t3735: F, t4142: F, t4149: F, t2820: F, t3751: F, t86: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11730 = t3820 * t1416;
    let t11736 = t1317 * t3838;
    let t11746 = 0.12841111111111111111e-1 * t11407;
    let t11767 = t1098 * t3843;
    let t11776 = t4277 * sigma2;
    let t11780 = t3728 * t3739;
    let t11782 = t4108 * t1466;
    let t11783 = t11782 * sigma2;
    let t11799 = t3728 * t3735;
    let t11811 = t4142 * t4149;
    let t11814 = t86 * t2820 * t3751;
    (t11730, t11736, t11746, t11767, t11776, t11780, t11782, t11783, t11799, t11811, t11814)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1009/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1009<F: Float>(t4277: F, t3728: F, t3739: F, t1466: F, t4108: F, t3735: F, t4142: F, t4149: F, t2820: F, t3751: F, t86: F, t4155: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t11776 = t4277 * sigma2;
    let t11780 = t3728 * t3739;
    let t11782 = t4108 * t1466;
    let t11783 = t11782 * sigma2;
    let t11799 = t3728 * t3735;
    let t11811 = t4142 * t4149;
    let t11814 = t86 * t2820 * t3751;
    let t11815 = t11814 * t4155;
    (t11776, t11780, t11782, t11783, t11799, t11811, t11815)
}

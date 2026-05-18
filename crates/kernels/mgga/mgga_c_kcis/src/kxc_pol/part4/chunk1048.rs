//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1048/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1048<F: Float>(t1014: F, t4789: F, t2820: F, t4552: F, t86: F, t4557: F, t4792: F, t9415: F, t3200: F, t1133: F, t167: F, t3211: F) -> (F, F, F, F, F) {
    let t13238 = t1014 * t4789;
    let t13241 = t86 * t2820 * t4552;
    let t13242 = t13241 * t4557;
    let t13243 = F::new(0.3684876543209876543e-2) * t13242;
    let t13246 = t9415 * t4792;
    let t13247 = t3200 * t13246;
    let t13249 = t167 * t1133;
    let t13250 = t3211 * t13249;
    (t13238, t13242, t13243, t13247, t13250)
}

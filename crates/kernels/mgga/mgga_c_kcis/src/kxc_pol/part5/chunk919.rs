//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 919/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk919<F: Float>(t4552: F, t4992: F, t86: F, t1014: F, t4789: F, t2820: F, t4557: F, t4807: F, t9429: F, t2861: F, t4778: F, t4797: F, t1769: F, t9528: F, t5020: F, t5010: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13199 = t86 * t4992 * t4552;
    let t13238 = t1014 * t4789;
    let t13241 = t86 * t2820 * t4552;
    let t13242 = t13241 * t4557;
    let t13243 = 0.3684876543209876543e-2 * t13242;
    let t13270 = t9429 * t4807;
    let t13271 = 0.14739506172839506172e-2 * t13270;
    let t13277 = t2861 * t4778;
    let t13278 = 0.33163888888888888888e-2 * t13277;
    let t13301 = t9429 * t4797;
    let t13302 = 0.14739506172839506172e-2 * t13301;
    let t13303 = t9528 * t1769;
    let t13305 = t2861 * t5020;
    let t13307 = t2861 * t5010;
    (t13199, t13238, t13242, t13243, t13270, t13271, t13277, t13278, t13301, t13302, t13303, t13305, t13307)
}

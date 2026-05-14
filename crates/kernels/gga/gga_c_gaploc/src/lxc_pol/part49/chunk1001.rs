//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1001/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1001<F: Float>(t41831: F, t41834: F, t41837: F, t41841: F, t41844: F, t41846: F, t41847: F, t41848: F, t41849: F, t41850: F, t41851: F, t41852: F, t40192: F, t40196: F, t12054: F, t9333: F) -> (F, F, F, F) {
    let t47923 = t41831 + t41834 - t41837 - 0.71500979903700853338e0 * t41841 + t41844 + t41846 + t41847 - t41848 + t41849 - t41850 - t41851 + t41852;
    let t47925 = 0.38342925953920749677e0 * t40192;
    let t47926 = 0.85206502119823888171e-1 * t40196;
    let t47927 = t12054 * t9333;
    (t47923, t47925, t47926, t47927)
}

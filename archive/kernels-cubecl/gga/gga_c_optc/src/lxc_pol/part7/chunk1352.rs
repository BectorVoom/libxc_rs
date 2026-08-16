//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1352/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1352<F: Float>(t3126: F, t8487: F, t3132: F, t4357: F, t24502: F, t465: F, t8970: F, t26911: F, t3133: F, t4386: F, t8493: F, t9189: F) -> (F, F, F, F, F) {
    let t26936 = t8487 * t3126;
    let t26938 = t3132 * t26936 * t4357;
    let t26940 = t465 * t24502;
    let t26941 = t26940 * t8970;
    let t26944 = t3132 * t26911 * t3133;
    let t26947 = t4386 * t9189 * t8493;
    (t26936, t26938, t26941, t26944, t26947)
}

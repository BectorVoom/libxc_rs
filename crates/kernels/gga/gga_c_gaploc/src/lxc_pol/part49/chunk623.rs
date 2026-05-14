//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 623/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk623<F: Float>(t3025: F, t9972: F, t8634: F, t955: F, t3464: F, t773: F, t1: F, t3431: F, t106: F, t316: F) -> (F, F, F, F, F, F) {
    let t10993 = 0.10725146985555128001e1 * t3025 * t9972;
    let t10995 = 0.35750489951850426669e0 * t955 * t8634;
    let t10996 = t773 * t3464;
    let t10999 = t3431 * t1;
    let t11000 = t10999 * t106;
    let t11001 = t11000 * t316;
    (t10993, t10995, t10996, t10999, t11000, t11001)
}

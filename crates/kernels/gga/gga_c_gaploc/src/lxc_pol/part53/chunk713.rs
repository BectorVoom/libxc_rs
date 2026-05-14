//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 713/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk713<F: Float>(t2679: F, t3243: F, t9796: F, t3255: F, t2617: F, t7803: F, t22980: F, t2615: F, t9438: F, t2365: F, t28648: F, t7630: F, t28431: F, t787: F, t9824: F, t22984: F, t7584: F) -> (F, F, F, F, F, F, F, F) {
    let t41136 = t9796 * t3243 * t2679;
    let t41139 = t9796 * t3255 * t2679;
    let t41143 = t7803 * t3243 * t2617;
    let t41231 = t2615 * t9438 * t22980;
    let t41234 = t7630 * t2365 * t28648;
    let t41236 = t787 * t28431;
    let t41237 = t41236 * t9824;
    let t41244 = t7584 * t9438 * t22984;
    (t41136, t41139, t41143, t41231, t41234, t41236, t41237, t41244)
}

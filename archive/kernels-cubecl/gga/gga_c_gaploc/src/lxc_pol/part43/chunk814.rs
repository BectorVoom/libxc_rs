//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 814/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk814<F: Float>(t2365: F, t28648: F, t7630: F, t28431: F, t787: F, t9824: F, t22984: F, t7584: F, t9438: F, t28983: F, t959: F, t28846: F) -> (F, F, F, F, F, F) {
    let t41234 = t7630 * t2365 * t28648;
    let t41236 = t787 * t28431;
    let t41237 = t41236 * t9824;
    let t41244 = t7584 * t9438 * t22984;
    let t41281 = t28983 * t959;
    let t41283 = t28846 * t959;
    (t41234, t41236, t41237, t41244, t41281, t41283)
}

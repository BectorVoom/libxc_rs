//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1118/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1118<F: Float>(t1: F, t1891: F, t7492: F, t2274: F, t7982: F, t2367: F, t8156: F, t930: F, t509: F, t896: F, t2724: F, t2812: F, t8040: F, t8143: F, t2723: F, t7178: F) -> (F, F, F, F, F, F) {
    let t25388 = t7492 * t1891 * t1;
    let t25401 = t7982 * t2274;
    let t25406 = t930 * t2367 * t8156;
    let t25412 = t509 * t896;
    let t25414 = t2812 * t25412 * t2724;
    let t25417 = t2812 * t8143 * t8040;
    let t25419 = t7178 * t2723;
    (t25388, t25401, t25406, t25414, t25417, t25419)
}

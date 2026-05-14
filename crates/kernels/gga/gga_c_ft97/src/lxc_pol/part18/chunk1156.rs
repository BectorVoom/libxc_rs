//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1156/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1156<F: Float>(t25990: F, t379: F, t22958: F, t5674: F, t469: F, t7954: F, t100248: F, t11437: F, t22986: F, t22953: F, t11594: F, t5691: F, t25878: F, t11604: F, t23031: F, t25928: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t100370 = t25990 * t379;
    let t100372 = t5674 * t22958 * t100370;
    let t100374 = t7954 * t469;
    let t100376 = t5674 * t100374 * t100248;
    let t100378 = t22986 * t11437;
    let t100380 = t5674 * t22953 * t100378;
    let t100382 = t5691 * t11594;
    let t100384 = t25878 * t22958 * t100382;
    let t100386 = t23031 * t11604;
    let t100388 = t25878 * t22953 * t100386;
    let t100390 = t22986 * t11604;
    let t100392 = t25878 * t25928 * t100390;
    (t100370, t100372, t100376, t100378, t100380, t100382, t100384, t100386, t100388, t100390, t100392)
}

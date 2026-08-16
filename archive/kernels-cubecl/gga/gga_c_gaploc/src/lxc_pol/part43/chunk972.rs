//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 972/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk972<F: Float>(t47143: F, t825: F, t969: F, t2365: F, t39149: F, t7390: F, t47294: F, t7584: F, t7585: F, t10930: F, t10931: F, t47243: F) -> (F, F, F, F) {
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    let t47357 = t7584 * t7585 * t47294;
    let t47360 = t10930 * t10931 * t47243;
    (t47344, t47347, t47357, t47360)
}

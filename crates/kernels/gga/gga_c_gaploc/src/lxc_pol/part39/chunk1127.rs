//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1127/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1127<F: Float>(t47294: F, t7584: F, t7585: F, t10930: F, t10931: F, t47243: F, t11016: F, t12256: F, t12207: F, t2718: F, t38947: F, t955: F) -> (F, F, F, F, F) {
    let t47357 = t7584 * t7585 * t47294;
    let t47360 = t10930 * t10931 * t47243;
    let t47362 = t12256 * t11016;
    let t47364 = t2718 * t12207;
    let t47366 = t955 * t38947;
    (t47357, t47360, t47362, t47364, t47366)
}

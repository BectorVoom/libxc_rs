//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 633/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk633<F: Float>(t1595: F, t930: F, t7914: F, t3056: F, t383: F, t35: F, t1594: F, t1608: F, t1614: F, t3070: F, t3066: F, t7839: F, t3037: F, t3029: F, t7847: F, t7858: F, t7906: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11084 = t930 * t1595;
    let t11085 = t7914 * t11084;
    let t11088 = t3056 * t383;
    let t11089 = t11088 * t35;
    let t11090 = t1594 * t11089;
    let t11094 = t1608 * t3070 * t1614;
    let t11095 = t3066 * t7839;
    let t11098 = t3037 * t7839;
    let t11104 = t3029 * t7847;
    let t11109 = t7906 * t7858;
    (t11084, t11085, t11088, t11089, t11090, t11094, t11095, t11098, t11104, t11109)
}

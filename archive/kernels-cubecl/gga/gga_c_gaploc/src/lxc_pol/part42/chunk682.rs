//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 682/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk682<F: Float>(t13149: F, t9438: F, t825: F, t10924: F, t2558: F, t9647: F, t10628: F, t5539: F, t10697: F, t3247: F, t11167: F, t2325: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13258 = t2325 * t883 * t11167;
    (t13150, t13151, t13182, t13183, t13194, t13195, t13200, t13201, t13258)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 471/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk471<F: Float>(t2089: F, t911: F, t1: F, t7284: F, t1422: F, t6109: F, t787: F, t1984: F, t7426: F, t201: F) -> (F, F, F, F, F) {
    let t7428 = t911 * t2089;
    let t7442 = t7284 * t1;
    let t7512 = t6109 * t1422;
    let t7513 = t787 * t7512;
    let t7572 = t1984 * t7426;
    let t7573 = t201 * t2089;
    (t7428, t7442, t7513, t7572, t7573)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 883/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk883<F: Float>(t28648: F, t5539: F, t7064: F, t28652: F, t9647: F, t1843: F, t28302: F, t28703: F, t883: F, t2537: F, t2558: F, t28431: F) -> (F, F, F, F, F, F) {
    let t40828 = t7064 * t5539 * t28648;
    let t40833 = t9647 * t5539 * t28652;
    let t40836 = t7064 * t1843 * t28302;
    let t40848 = t883 * t28703;
    let t40850 = t9647 * t2537 * t40848;
    let t40853 = t9647 * t28431 * t2558;
    (t40828, t40833, t40836, t40848, t40850, t40853)
}

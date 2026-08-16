//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 357/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk357<F: Float>(t169: F, t2925: F, t299: F, t706: F, t1022: F, t296: F, t123: F, t734: F, t795: F) -> (F, F, F, F, F, F) {
    let t2926 = t2925 * t169;
    let t2927 = t2926 * t299;
    let t2928 = t706 * t2927;
    let t2931 = t296 * t1022;
    let t2932 = t2931 * t123;
    let t2933 = t2932 * t734;
    let t2936 = t795 * t1022;
    (t2927, t2928, t2931, t2932, t2933, t2936)
}

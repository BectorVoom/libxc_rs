//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 807/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk807<F: Float>(t13150: F, t2013: F, t10007: F, t2925: F, t825: F, t9438: F, t3039: F, t5774: F, t24549: F, t7584: F, t13072: F, t32757: F) -> (F, F, F, F, F) {
    let t44084 = t2013 * t13150;
    let t44088 = t825 * t9438 * t10007 * t2925;
    let t44090 = t3039 * t5774;
    let t44117 = t7584 * t9438 * t24549;
    let t44130 = t32757 * t13072;
    (t44084, t44088, t44090, t44117, t44130)
}

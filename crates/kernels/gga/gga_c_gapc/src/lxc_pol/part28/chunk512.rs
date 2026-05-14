//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 512/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk512<F: Float>(t3103: F, t581: F, t1720: F, t458: F, t178: F, t568: F, t116: F, t19: F, t147: F, t5: F, t3071: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3104 = t581 * t3103;
    let t3105 = t1720 * t458;
    let t3106 = t3104 * t3105;
    let t3108 = t178 * t3103;
    let t3109 = t1720 * t568;
    let t3110 = t3108 * t3109;
    let t3112 = t116 * t19;
    let t3113 = t147 * t5;
    let t3114 = t3113 * t3071;
    (t3104, t3105, t3106, t3108, t3109, t3110, t3112, t3113, t3114)
}

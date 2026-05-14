//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 980/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk980<F: Float>(t1381: F, t2353: F, t501: F, t8040: F, t1959: F, t2967: F, t747: F, t9032: F, t1022: F, t5501: F, t835: F, t8720: F, t2975: F, t5679: F, t1710: F, t2158: F, t3039: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24215 = t2353 * t1381;
    let t24282 = t8040 * t501;
    let t24295 = t2967 * t1959;
    let t24303 = t9032 * t747;
    let t24321 = t5501 * t1022;
    let t24339 = t835 * t8720;
    let t24344 = t5679 * t2975;
    let t24350 = t1022 * t1710;
    let t24364 = t3039 * t2158;
    (t24215, t24282, t24295, t24303, t24321, t24339, t24344, t24350, t24364)
}

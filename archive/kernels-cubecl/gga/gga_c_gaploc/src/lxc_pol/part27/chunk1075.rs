//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1075/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1075<F: Float>(t1022: F, t5501: F, t835: F, t8720: F, t2975: F, t5679: F, t1710: F, t2158: F, t3039: F, t783: F, t8633: F, t1835: F) -> (F, F, F, F, F, F, F) {
    let t24321 = t5501 * t1022;
    let t24339 = t835 * t8720;
    let t24344 = t5679 * t2975;
    let t24350 = t1022 * t1710;
    let t24364 = t3039 * t2158;
    let t24390 = t8633 * t783;
    let t24446 = t1022 * t1835;
    (t24321, t24339, t24344, t24350, t24364, t24390, t24446)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 633/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk633<F: Float>(t12903: F, t12933: F, t12956: F, t12999: F, t502: F, t3263: F, t8862: F, t2969: F, t3322: F, t10800: F, t977: F, t11004: F, t935: F, t1445: F, t2087: F, t3009: F, t3234: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13001 = t12903 + t12933 + t12956 + t12999;
    let t13002 = t502 * t13001;
    let t13004 = 2.0 * t8862 * t3263;
    let t13005 = t2969 * t3322;
    let t13006 = t10800 * t977;
    let t13008 = t11004 * t935;
    let t13009 = t1445 * t13008;
    let t13010 = t2087 * t13009;
    let t13012 = t3009 * t3234;
    (t13001, t13002, t13004, t13005, t13006, t13008, t13009, t13010, t13012)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 618/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk618<F: Float>(t2869: F, t521: F, t1128: F, t2893: F, t2889: F, t1139: F, t513: F, t1143: F) -> (F, F, F, F, F, F) {
    let t2904 = t521 * t2869;
    let t2910 = t1128 * t2893;
    let t2913 = t521 * t2889;
    let t2916 = t1139 * t2893;
    let t2919 = t513 * t2889;
    let t2922 = t1143 * t1139;
    (t2904, t2910, t2913, t2916, t2919, t2922)
}

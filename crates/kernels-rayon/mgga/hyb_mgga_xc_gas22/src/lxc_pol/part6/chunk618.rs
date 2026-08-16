//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 618/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk618(t2869: f64, t521: f64, t1128: f64, t2893: f64, t2889: f64, t1139: f64, t513: f64, t1143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2904 = t521 * t2869;
    let t2910 = t1128 * t2893;
    let t2913 = t521 * t2889;
    let t2916 = t1139 * t2893;
    let t2919 = t513 * t2889;
    let t2922 = t1143 * t1139;
    (t2904, t2910, t2913, t2916, t2919, t2922)
}

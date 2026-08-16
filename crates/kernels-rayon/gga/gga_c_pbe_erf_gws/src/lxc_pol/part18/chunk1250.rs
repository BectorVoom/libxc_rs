//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1250/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1250(t14784: f64, t50994: f64, t20091: f64, t4157: f64, t3202: f64, t3955: f64, t14113: f64, t14614: f64, t2242: f64, t4161: f64, t14742: f64, t840: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53952 = t50994 * t14784;
    let t53953 = 7.0_f64 / 288.0_f64 * t53952;
    let t53959 = t20091 * t4157;
    let t53970 = t3955 * t3202;
    let t53971 = 7.0_f64 / 144.0_f64 * t53970;
    let t53975 = t14113 * t14614;
    let t53976 = 7.0_f64 / 576.0_f64 * t53975;
    let t53977 = t2242 * t4161;
    let t53980 = 7.0_f64 / 144.0_f64 * t840 * t14742;
    (t53953, t53959, t53971, t53976, t53977, t53980)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2243;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta504(t12050: f64, t3151: f64, t357: f64, t15907: f64, t3133: f64, t3302: f64, t4893: f64, t3059: f64, t4975: f64, t4781: f64, t12132: f64, t1647: f64, t3316: f64, t1083: f64, t12122: f64, t12127: f64, t12146: f64, t12149: f64, t12154: f64, t15655: f64, t16529: f64, t16534: f64, t16537: f64, t16540: f64, t16544: f64, t16552: f64, t16555: f64, t16559: f64, t16562: f64, t16566: f64, t3278: f64, t3288: f64, t3309: f64, t3319: f64, t342: f64, t4954: f64, t4964: f64, t4977: f64, t4981: f64, t4996: f64, t5009: f64, t16423: f64, t16475: f64, t16526: f64, t1079: f64, t1071: f64, t4746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242(t12050, t3151, t357, t15907, t3133, t3302, t4893, t3059, t4975, t4781, t12132, t1647, t3316);
        let t16589 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2243(t1083, t12122, t12127, t12146, t12149, t12154, t15655, t16529, t16534, t16537, t16540, t16544, t16552, t16555, t16559, t16562, t16566, t16569, t16574, t16578, t16581, t16584, t3278, t3288, t3309, t3319, t342, t4954, t4964, t4977, t4981, t4996, t5009);
        let (t16591, t16592, t16597) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2244(t16423, t16475, t16526, t16589, t1079, t1071, t4746);
    (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584, t16591, t16592, t16597)
}

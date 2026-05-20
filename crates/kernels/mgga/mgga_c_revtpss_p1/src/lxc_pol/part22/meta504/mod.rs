//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2243;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta504<F: Float>(t12050: F, t3151: F, t357: F, t15907: F, t3133: F, t3302: F, t4893: F, t3059: F, t4975: F, t4781: F, t12132: F, t1647: F, t3316: F, t1083: F, t12122: F, t12127: F, t12146: F, t12149: F, t12154: F, t15655: F, t16529: F, t16534: F, t16537: F, t16540: F, t16544: F, t16552: F, t16555: F, t16559: F, t16562: F, t16566: F, t3278: F, t3288: F, t3309: F, t3319: F, t342: F, t4954: F, t4964: F, t4977: F, t4981: F, t4996: F, t5009: F, t16423: F, t16475: F, t16526: F, t1079: F, t1071: F, t4746: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242::<F>(t12050, t3151, t357, t15907, t3133, t3302, t4893, t3059, t4975, t4781, t12132, t1647, t3316);
        let t16589 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2243::<F>(t1083, t12122, t12127, t12146, t12149, t12154, t15655, t16529, t16534, t16537, t16540, t16544, t16552, t16555, t16559, t16562, t16566, t16569, t16574, t16578, t16581, t16584, t3278, t3288, t3309, t3319, t342, t4954, t4964, t4977, t4981, t4996, t5009);
        let (t16591, t16592, t16597) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2244::<F>(t16423, t16475, t16526, t16589, t1079, t1071, t4746);
    (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584, t16591, t16592, t16597)
}

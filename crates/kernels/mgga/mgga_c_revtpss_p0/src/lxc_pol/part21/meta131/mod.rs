//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk846;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk847;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk848;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk849;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk850;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta131<F: Float>(t3090: F, t3114: F, t373: F, t66: F, t828: F, t1043: F, t999: F, t1045: F, t1032: F, t989: F, t1040: F, t1024: F, t1062: F, t1065: F, t906: F, t1042: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3115 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk845::<F>(t3090, t3114);
        let t3116 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk846::<F>(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk847::<F>(t3116, t828);
        let (t3118, t3119, t3120) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk848::<F>(t1043, t999, t1045, t3117);
        let (t3123, t3124) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk849::<F>(t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk850::<F>(t1024, t1062);
        let (t3128, t3129, t3130) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk851::<F>(t1065, t999, t906, t1042);
    (t3115, t3116, t3117, t3118, t3119, t3120, t3123, t3124, t3127, t3128, t3129, t3130)
}

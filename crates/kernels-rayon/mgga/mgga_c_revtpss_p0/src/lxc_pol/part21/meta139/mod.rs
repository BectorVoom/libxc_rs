//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk892;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk893;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk894;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk895;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk896;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk897;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta139(t3286: f64, t994: f64, t1089: f64, t3118: f64, t1071: f64, t359: f64, t999: f64, t1082: f64, t3075: f64, t3140: f64, t3143: f64, t342: f64, t3151: f64, t378: f64, t335: f64, t368: f64, t3153: f64, t3154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3287 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk892(t3286, t994);
        let t3288 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk893(t1089, t3118);
        let t3291 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk894(t1071, t359);
        let (t3292, t3295, t3298) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk895(t3291, t999, t1082, t3075, t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk896(t3298, t342);
        let (t3300, t3302) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk897(t3151, t378, t335, t368);
        let (t3303, t3304) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk898(t3153, t3302, t3154);
    (t3287, t3288, t3291, t3292, t3295, t3298, t3299, t3300, t3302, t3303, t3304)
}

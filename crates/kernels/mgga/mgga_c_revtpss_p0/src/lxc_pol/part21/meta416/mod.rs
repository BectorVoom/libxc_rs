//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1893;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1894;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1895;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta416<F: Float>(t13392: F, t70: F, t4186: F, t606: F, t4181: F, t627: F, t13312: F, t36: F, t4187: F, t1470: F, t2291: F, t13389: F, t1494: F, t2292: F, t4182: F, t4188: F, t4191: F, t4238: F, t628: F, t641: F, t71: F, t85: F, t13363: F, t10298: F, t10301: F, t10309: F, t13267: F, t13269: F, t13272: F, t13283: F, t13286: F, t13289: F, t1497: F, t2242: F, t2247: F, t2248: F, t2315: F, t4173: F, t4178: F, t4241: F, t603: F, t644: F, t91: F, t5: F, t117: F, t116: F, t4245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13393, t13396) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1893::<F>(t13392, t70, t4186, t606);
        let (t13397, t13400, t13405, t13406, t13409, t13414, t13419) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1894::<F>(t13396, t70, t4181, t627, t13312, t36, t4187, t1470, t2291, t13389, t13393, t1494, t2292, t4182, t4188, t4191, t4238, t628, t641, t71, t85);
        let (t13420, t13423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1895::<F>(t13363, t13419, t10298, t10301, t10309, t13267, t13269, t13272, t13283, t13286, t13289, t1497, t2242, t2247, t2248, t2315, t4173, t4178, t4241, t603, t644, t91);
        let (t13424, t13425, t13426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1896::<F>(t5, t13423, t117, t116, t4245);
    (t13393, t13396, t13397, t13400, t13405, t13406, t13409, t13414, t13420, t13424, t13425, t13426)
}

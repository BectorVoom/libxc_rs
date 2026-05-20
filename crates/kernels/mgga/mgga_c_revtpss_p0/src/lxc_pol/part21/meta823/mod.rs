//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta823 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3058;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3059;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3060;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3061;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3062;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3064;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta823<F: Float>(t13312: F, t3362: F, t606: F, t128: F, t3360: F, t16724: F, t2258: F, t12268: F, t2251: F, t4186: F, t10326: F, t5046: F, t16726: F, t689: F, t43830: F, t43832: F, t43995: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56176: F, t56181: F, t56184: F, t56185: F, t56187: F, t56189: F, t16730: F, t16721: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56192, t56194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3058::<F>(t13312, t3362, t606, t128, t3360);
        let (t56196, t56198) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3059::<F>(t16724, t2258, t128, t3360);
        let (t56201, t56203) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3060::<F>(t12268, t2251, t4186, t128, t3360);
        let (t56205, t56207) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3061::<F>(t10326, t5046, t128, t3360);
        let t56209 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3062::<F>(t16726, t689);
        let t56211 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063::<F>(t43830, t43832, t43995, t56151, t56155, t56159, t56163, t56167, t56174, t56176, t56181, t56184, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t56212 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3064::<F>(t16730, t689);
        let t56214 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3065::<F>(t16721, t689);
    (t56192, t56194, t56196, t56198, t56201, t56203, t56205, t56207, t56209, t56211, t56212, t56214)
}

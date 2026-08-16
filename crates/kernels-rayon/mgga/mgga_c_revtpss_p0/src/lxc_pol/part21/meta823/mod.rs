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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3058;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3059;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3060;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3061;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3062;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3064;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta823(t13312: f64, t3362: f64, t606: f64, t128: f64, t3360: f64, t16724: f64, t2258: f64, t12268: f64, t2251: f64, t4186: f64, t10326: f64, t5046: f64, t16726: f64, t689: f64, t43830: f64, t43832: f64, t43995: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56176: f64, t56181: f64, t56184: f64, t56185: f64, t56187: f64, t56189: f64, t16730: f64, t16721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56192, t56194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3058(t13312, t3362, t606, t128, t3360);
        let (t56196, t56198) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3059(t16724, t2258, t128, t3360);
        let (t56201, t56203) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3060(t12268, t2251, t4186, t128, t3360);
        let (t56205, t56207) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3061(t10326, t5046, t128, t3360);
        let t56209 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3062(t16726, t689);
        let t56211 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3063(t43830, t43832, t43995, t56151, t56155, t56159, t56163, t56167, t56174, t56176, t56181, t56184, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t56212 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3064(t16730, t689);
        let t56214 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3065(t16721, t689);
    (t56192, t56194, t56196, t56198, t56201, t56203, t56205, t56207, t56209, t56211, t56212, t56214)
}

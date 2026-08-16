//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta840(t16862: f64, t3399: f64, t12322: f64, t5087: f64, t12328: f64, t1723: f64, t43821: f64, t43946: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t43881: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64, t1139: f64, t43828: f64, t43911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58055, t58057, t58060, t58063, t58084) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149(t16862, t3399, t12322, t5087, t12328, t1723, t43821, t43946, t56176, t56183, t43830, t43832, t43881, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t58105 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t58106, t58107, t58116) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151(t58084, t58105, t1139, t56176, t43828, t43830, t43832, t43911, t56174, t56181, t58055, t58057, t58060, t58063);
        let t58129 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3152(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
    (t58055, t58057, t58060, t58063, t58106, t58107, t58116, t58129)
}

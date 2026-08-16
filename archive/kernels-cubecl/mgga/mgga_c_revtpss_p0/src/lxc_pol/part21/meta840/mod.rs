//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta840<F: Float>(t16862: F, t3399: F, t12322: F, t5087: F, t12328: F, t1723: F, t43821: F, t43946: F, t56176: F, t56183: F, t43830: F, t43832: F, t43881: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F, t1139: F, t43828: F, t43911: F) -> (F, F, F, F, F, F, F, F) {
        let (t58055, t58057, t58060, t58063, t58084) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149::<F>(t16862, t3399, t12322, t5087, t12328, t1723, t43821, t43946, t56176, t56183, t43830, t43832, t43881, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t58105 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150::<F>(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t58106, t58107, t58116) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3151::<F>(t58084, t58105, t1139, t56176, t43828, t43830, t43832, t43911, t56174, t56181, t58055, t58057, t58060, t58063);
        let t58129 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3152::<F>(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
    (t58055, t58057, t58060, t58063, t58106, t58107, t58116, t58129)
}

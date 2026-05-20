//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1747;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta368<F: Float>(t3059: F, t3291: F, t4980: F, t994: F, t3151: F, t999: F, t3304: F, t4995: F, t3318: F, t1043: F, t3153: F, t3133: F, t4982: F, t1071: F, t1089: F, t3046: F, t3286: F, t3057: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12119, t12122) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1744::<F>(t3059, t3291, t4980, t994);
        let (t12123, t12124, t12127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1745::<F>(t3151, t999, t3304, t4995, t994);
        let (t12128, t12131) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1746::<F>(t12123, t3318, t1043, t3153);
        let (t12132, t12133, t12137, t12143, t12146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1747::<F>(t3133, t4982, t12131, t1071, t1089, t999, t3046, t3286);
        let t12149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1748::<F>(t3057, t3286);
    (t12119, t12122, t12124, t12127, t12128, t12131, t12132, t12133, t12137, t12143, t12146, t12149)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta238 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1076;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1077;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1078;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1079;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta238<F: Float>(t25: F, t28: F, t1408: F, t3664: F, t514: F, t5397: F, t1649: F, t3672: F, t517: F, t5966: F, t157: F, zeta_threshold: F, t182: F, t2408: F, t2417: F, t2423: F, t2426: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t6299: F, t6300: F, t6301: F, t6304: F, t1845: F, t184: F, t17: F, t1799: F, t1298: F, t3704: F, t1302: F, t3711: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6305, t6312, t6320) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1076::<F>(t25, t28, t1408, t3664, t514, t5397, t1649, t3672, t517, t5966, t157, zeta_threshold);
        let (t6322, t6323) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1077::<F>(t182, t6320, t2408, t2417, t2423, t2426, t3686, t3688, t3690, t3695, t3813, t3918, t6299, t6300, t6301, t6304);
        let t6324 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1078::<F>(t1845);
        let (t6328, t6329, t6330) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1079::<F>(t184, t6320, t17, t1799);
        let t6347 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1080::<F>(t25, t28, t1298, t3704, t5397, t6305, t1302, t3711, t5966, t6312, zeta_threshold);
    (t6305, t6312, t6320, t6322, t6323, t6324, t6328, t6329, t6330, t6347)
}

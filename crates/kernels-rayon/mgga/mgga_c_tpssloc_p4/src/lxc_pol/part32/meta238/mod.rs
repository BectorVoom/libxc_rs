//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1076;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1077;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1078;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1079;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta238(t25: f64, t28: f64, t1408: f64, t3664: f64, t514: f64, t5397: f64, t1649: f64, t3672: f64, t517: f64, t5966: f64, t157: f64, zeta_threshold: f64, t182: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t3918: f64, t6299: f64, t6300: f64, t6301: f64, t6304: f64, t1845: f64, t184: f64, t17: f64, t1799: f64, t1298: f64, t3704: f64, t1302: f64, t3711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6305, t6312, t6320) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1076(t25, t28, t1408, t3664, t514, t5397, t1649, t3672, t517, t5966, t157, zeta_threshold);
        let (t6322, t6323) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1077(t182, t6320, t2408, t2417, t2423, t2426, t3686, t3688, t3690, t3695, t3813, t3918, t6299, t6300, t6301, t6304);
        let t6324 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1078(t1845);
        let (t6328, t6329, t6330) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1079(t184, t6320, t17, t1799);
        let t6347 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1080(t25, t28, t1298, t3704, t5397, t6305, t1302, t3711, t5966, t6312, zeta_threshold);
    (t6305, t6312, t6320, t6322, t6323, t6324, t6328, t6329, t6330, t6347)
}

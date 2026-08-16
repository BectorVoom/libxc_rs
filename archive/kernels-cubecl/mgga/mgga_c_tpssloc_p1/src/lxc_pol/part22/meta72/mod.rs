//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk504;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk505;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk506;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta72<F: Float>(t40: F, t52: F, t145: F, t1471: F, t185: F, t157: F, t182: F, t1409: F, t767: F, t771: F, zeta_threshold: F, t210: F, t214: F, t785: F, t787: F, t797: F, t252: F, t119: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1472, t1473, t1474, t1476, t1484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk504::<F>(t40, t52, t145, t1471, t185, t157, t182, t1409, t767, t771, zeta_threshold);
        let (t1489, t1492) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk505::<F>(t1484, t210, t214, t785, t787, t797);
        let (t1493, t1495) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk506::<F>(t1492, t252, t119, t1484);
        let (t1496, t1499) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk507::<F>(t1495, t210, t1492, t225);
    (t1472, t1473, t1474, t1476, t1484, t1489, t1492, t1493, t1495, t1496, t1499)
}

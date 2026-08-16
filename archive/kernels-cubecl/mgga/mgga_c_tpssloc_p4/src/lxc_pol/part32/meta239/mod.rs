//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1081;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1082;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta239<F: Float>(t210: F, t214: F, t6330: F, t6347: F, t1315: F, t3725: F, t3731: F, t3733: F, t3751: F, t5192: F, t5203: F, t562: F, t1807: F, t1834: F, t119: F, t225: F, t554: F, t1824: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6353, t6358, t6361) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1081::<F>(t210, t214, t6330, t6347, t1315, t3725, t3731, t3733, t3751, t5192, t5203);
        let (t6362, t6364, t6370, t6371, t6374, t6375, t6378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1082::<F>(t562, t6361, t1807, t1834, t119, t6330, t210, t6347, t225);
        let (t6379, t6387) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1083::<F>(t554, t6378, t1824);
    (t6353, t6358, t6361, t6362, t6364, t6370, t6371, t6374, t6375, t6378, t6379, t6387)
}

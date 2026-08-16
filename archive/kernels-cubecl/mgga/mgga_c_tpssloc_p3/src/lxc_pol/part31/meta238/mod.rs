//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk994;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk995;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta238<F: Float>(t184: F, t6320: F, t17: F, t1799: F, t25: F, t28: F, t1298: F, t3704: F, t5397: F, t6305: F, t1302: F, t3711: F, t5966: F, t6312: F, zeta_threshold: F, t210: F, t214: F, t1315: F, t3725: F, t3731: F, t3733: F, t3751: F, t5192: F, t5203: F) -> (F, F, F, F, F, F, F) {
        let (t6328, t6329, t6330) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk994::<F>(t184, t6320, t17, t1799);
        let t6347 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk995::<F>(t25, t28, t1298, t3704, t5397, t6305, t1302, t3711, t5966, t6312, zeta_threshold);
        let (t6353, t6358, t6361) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk996::<F>(t210, t214, t6330, t6347, t1315, t3725, t3731, t3733, t3751, t5192, t5203);
    (t6328, t6329, t6330, t6347, t6353, t6358, t6361)
}

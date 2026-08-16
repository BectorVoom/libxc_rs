//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk994;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk995;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta238(t184: f64, t6320: f64, t17: f64, t1799: f64, t25: f64, t28: f64, t1298: f64, t3704: f64, t5397: f64, t6305: f64, t1302: f64, t3711: f64, t5966: f64, t6312: f64, zeta_threshold: f64, t210: f64, t214: f64, t1315: f64, t3725: f64, t3731: f64, t3733: f64, t3751: f64, t5192: f64, t5203: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t6328, t6329, t6330) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk994(t184, t6320, t17, t1799);
        let t6347 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk995(t25, t28, t1298, t3704, t5397, t6305, t1302, t3711, t5966, t6312, zeta_threshold);
        let (t6353, t6358, t6361) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk996(t210, t214, t6330, t6347, t1315, t3725, t3731, t3733, t3751, t5192, t5203);
    (t6328, t6329, t6330, t6347, t6353, t6358, t6361)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1059/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1059(t6880: f64, t7685: f64, t1266: f64, t1976: f64, t1980: f64, t26002: f64, t26005: f64, t26006: f64, t26098: f64, t26138: f64, t26141: f64, t26144: f64, t26145: f64, t26147: f64, t26150: f64, t4026: f64, t510: f64, t5361: f64, t574: f64, t7451: f64) -> f64 {
    let t26153 = 3.0_f64 * t7685 * t6880;
    let t26155 = -t1266 * t7451 - t1976 * t4026 + t1980 * t5361 - t26098 * t510 + t26138 * t574 - t26002 - t26005 - t26006 - t26141 - t26144 - t26145 + t26147 - t26150 + t26153;
    t26155
}

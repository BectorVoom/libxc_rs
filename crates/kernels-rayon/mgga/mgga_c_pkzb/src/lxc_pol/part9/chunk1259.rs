//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1259/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1259(t12: f64, t82: f64, t16129: f64, t1151: f64, t1153: f64, t20668: f64, t20698: f64, t21266: f64, t21284: f64, t21287: f64, t21309: f64, t21321: f64, t2159: f64, t2163: f64, t22129: f64, t3000: f64, t3005: f64, t318: f64, t319: f64, t6071: f64, t6078: f64, t7897: f64, t7909: f64, t808: f64, t810: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t22147 = 12.0_f64 * t82;
    let t22148 = 24.0_f64 * t16129;
    let t22149 = -t22147 + t22148;
    let t22150 = piecewise3(t84, 0.0_f64, t22149);
    let t22154 = piecewise3(t203, 0.0_f64, (t20668 + t20698 + t21266 + t21284 + t21287 + t21309 + t21321 + t22129) * t319 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t7897 * t810 + 3.0_f64 / 2.0_f64 * t3000 * t2163 + t1151 * t6078 / 2.0_f64 + t6071 * t1153 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2159 * t3005 + 3.0_f64 / 2.0_f64 * t808 * t7909 + t318 * t22150 / 2.0_f64);
    (t22149, t22154)
}

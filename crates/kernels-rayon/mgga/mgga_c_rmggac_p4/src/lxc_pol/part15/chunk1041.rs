//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1041/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1041(t40278: f64, t8443: f64, t1704: f64, t352: f64, t2186: f64, t9795: f64, t6491: f64, t668: f64, t7244: f64, t9985: f64, t3351: f64, t3352: f64, t511: f64, t6449: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47119 = t40278 * t8443;
    let t47124 = t1704 * t352;
    let t47133 = t2186 * t9795;
    let t47135 = t6491 * t668;
    let t47138 = t7244 * t9985;
    let t47142 = t3351 * t3352 * t511 * t6449;
    (t47119, t47124, t47133, t47135, t47138, t47142)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1081/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1081(t8852: f64, t8856: f64, t8860: f64, t8864: f64, t8867: f64, t8870: f64, t8874: f64, t8877: f64, t8879: f64, t8888: f64, t9032: f64, t9033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42289 = 0.30487649791575028314e-3_f64 * t8852;
    let t42290 = 0.30487649791575028314e-3_f64 * t8856;
    let t42291 = 0.30487649791575028314e-3_f64 * t8860;
    let t42292 = 0.30487649791575028314e-3_f64 * t8864;
    let t42293 = 0.11974241701863808564e0_f64 * t8867;
    let t42294 = 2.0_f64 * t8870;
    let t42296 = 0.79828278012425390428e-1_f64 * t8874;
    let t42297 = 0.4726e1_f64 * t8877;
    let t42298 = 0.11974241701863808564e0_f64 * t8879;
    let t42299 = 2.0_f64 * t8888;
    let t42300 = 2.0_f64 * t9032;
    let t42301 = 0.11974241701863808564e0_f64 * t9033;
    (t42289, t42290, t42291, t42292, t42293, t42294, t42296, t42297, t42298, t42299, t42300, t42301)
}

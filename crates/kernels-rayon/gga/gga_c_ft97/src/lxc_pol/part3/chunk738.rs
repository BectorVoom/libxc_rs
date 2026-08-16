//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 738/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk738(t1882: f64, t4173: f64, t4188: f64, t4178: f64, t4183: f64, t4267: f64, t8392: f64, t1526: f64, t4406: f64, t7705: f64, t339: f64, t39: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15471 = 2.0_f64 / 27.0_f64 * t1882 * t4173;
    let t15491 = 2.0_f64 / 27.0_f64 * t1882 * t4188;
    let t15500 = 2.0_f64 / 9.0_f64 * t1882 * t4178;
    let t15502 = 4.0_f64 / 9.0_f64 * t1882 * t4183;
    let t15532 = 4.0_f64 / 27.0_f64 * t8392 * t4267;
    let t15562 = t1526 * t7705 * t4406;
    let t15564 = t339 * t39;
    (t15471, t15491, t15500, t15502, t15532, t15562, t15564)
}

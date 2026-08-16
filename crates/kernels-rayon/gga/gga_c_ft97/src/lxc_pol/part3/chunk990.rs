//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 990/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk990(t19329: f64, t296: f64, t5374: f64, t870: f64, t875: f64, t4635: f64, t824: f64, t2875: f64, t2874: f64, t1882: f64, t5315: f64, t1248: f64, t15133: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19330 = t296 * t19329;
    let t19333 = t5374 * t870;
    let t19334 = t19333 * t875;
    let t19335 = t296 * t19334;
    let t19338 = t4635 * t824;
    let t19339 = t2875 * t19338;
    let t19340 = t2874 * t19339;
    let t19343 = t1882 * t5315;
    let t19345 = t15133 * t1248;
    (t19330, t19334, t19335, t19340, t19343, t19345)
}

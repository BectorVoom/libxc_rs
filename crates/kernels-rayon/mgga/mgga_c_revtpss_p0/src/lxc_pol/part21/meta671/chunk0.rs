//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2472/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472(t3057: f64, t4995: f64, t3143: f64, t42859: f64, t342: f64, t12032: f64, t359: f64, t3043: f64, t3298: f64, t16551: f64, t994: f64, t16558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43504 = t359 * t12032;
    let t43512 = t3043 * t3298;
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    (t43456, t43471, t43472, t43504, t43512, t43520, t43524)
}

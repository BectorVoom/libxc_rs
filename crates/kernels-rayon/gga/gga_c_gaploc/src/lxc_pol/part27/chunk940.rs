//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 940/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk940(t10314: f64, t6964: f64, t6963: f64, t2299: f64, t986: f64, t1415: f64, t1646: f64, t2877: f64, t9285: f64, t3390: f64, t4614: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10315 = t6964 * t10314;
    let t10317 = 0.71500979903700853338e0_f64 * t6963 * t10315;
    let t10318 = t2299 * t986;
    let t10319 = t1415 * t10318;
    let t10321 = 0.35750489951850426669e0_f64 * t10319 * t1646;
    let t10323 = 0.35750489951850426669e0_f64 * t9285 * t2877;
    let t10324 = t4614 * t3390;
    let t10326 = 0.61348681526273199483e1_f64 * t574 * t10324;
    (t10315, t10317, t10318, t10319, t10321, t10323, t10324, t10326)
}

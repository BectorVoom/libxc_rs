//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 713/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk713(t13555: f64, t2580: f64, t2508: f64, t13525: f64, t739: f64, t738: f64, t13195: f64, t13201: f64, t13226: f64, t13537: f64, t13539: f64, t13544: f64, t13547: f64, t13550: f64, t13554: f64, t270: f64) -> (f64, f64, f64, f64) {
    let t13556 = t2580 * t13555;
    let t13558 = 0.15381052460284448567e-1_f64 * t2508 * t13556;
    let t13559 = t739 * t13525;
    let t13560 = t738 * t13559;
    let t13566 = t13537 + 0.30762104920568897134e-1_f64 * t2508 * t13539 + t13544 - t13547 + t13550 - t13554 + t13558 - 0.76905262301422242837e-2_f64 * t270 * t13560 + 0.2563508743380741428e-2_f64 * t13195 - 0.3845263115071112142e-2_f64 * t13201 - 0.1281754371690370714e-2_f64 * t13226;
    (t13556, t13559, t13560, t13566)
}

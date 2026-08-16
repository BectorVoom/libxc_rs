//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 876/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk876(t15742: f64, t3613: f64, t12137: f64, t15737: f64, t15746: f64, t2266: f64, t3653: f64, t925: f64, t2253: f64, t4874: f64, t4885: f64, t1073: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17613 = t3613 * t15742;
    let t17616 = t12137 * t15737;
    let t17619 = t3613 * t15746;
    let t17623 = t2266 * t925 * t3653;
    let t17626 = t2253 * t4874;
    let t17627 = t2253 * t4885;
    let t17630 = t920 * t1073;
    (t17613, t17616, t17619, t17623, t17626, t17627, t17630)
}

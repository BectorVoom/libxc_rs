//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 732/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk732(t161: f64, t4576: f64, t1: f64, t1368: f64, t3: f64, t19: f64, t545: f64, t20: f64, t1365: f64, t1472: f64, t1372: f64, t1375: f64, t1379: f64, t1380: f64, t159: f64, t39: f64, t696: f64, t697: f64) -> (f64, f64, f64, f64) {
    let t4577 = t4576 * t161;
    let t4579 = t1368 * t1;
    let t4580 = t4579 * t3;
    let t4585 = t545 * t19;
    let t4586 = t4585 * t20;
    let t4589 = t1365 * t161;
    let t4592 = t1472 * t161;
    let t4598 = t4577 / 2.0_f64 + 0.9405e-1_f64 * t4580 * t697 - 0.1254e0_f64 * t1372 * t1375 + 0.2358774e-1_f64 * t4586 * t1380 + 0.97533333333333333333e-1_f64 * t696 * t4589 - 0.3145032e-1_f64 * t1379 * t4592 + 0.18830592773509979209e-2_f64 * t159 * t39 * t161;
    (t4577, t4579, t4585, t4598)
}

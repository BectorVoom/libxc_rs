//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1204/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1204(t19898: f64, t3912: f64, t4384: f64, t1161: f64, t26654: f64, t1114: f64, t3747: f64, t4383: f64, t12227: f64, t331: f64, t11387: f64, t11660: f64, t6472: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35000 = t3912 * t19898;
    let t35003 = t3912 * t4384;
    let t35023 = t26654 * t1161;
    let t35057 = t1114 * t3747 * t4383;
    let t35171 = t12227 * t331;
    let t35187 = t11387 * t331;
    let t35193 = t11660 * t6472 * t35171;
    (t35000, t35003, t35023, t35057, t35187, t35193)
}

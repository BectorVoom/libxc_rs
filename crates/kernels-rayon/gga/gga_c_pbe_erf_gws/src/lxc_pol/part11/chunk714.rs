//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 714/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk714(t2654: f64, t5390: f64, t3603: f64, t735: f64, t3342: f64, t476: f64, t3351: f64, t478: f64, t1651: f64, t3503: f64, t587: f64, t3562: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10633 = 0.2e-20_f64 * t2654 * t5390;
    let t10634 = t3603 * t735;
    let t10636 = t476 * t3342;
    let t10646 = t478 * t3351;
    let t10685 = t1651 * t3503;
    let t10686 = t587 * t10685;
    let t10691 = t649 * t3562;
    (t10633, t10634, t10636, t10646, t10685, t10686, t10691)
}

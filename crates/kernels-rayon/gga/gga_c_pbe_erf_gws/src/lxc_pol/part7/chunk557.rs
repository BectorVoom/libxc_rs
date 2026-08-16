//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 557/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk557(t331: f64, t641: f64, t589: f64, t181: f64, t562: f64, t184: f64, t1640: f64, t219: f64) -> (f64, f64, f64, f64, f64) {
    let t2591 = t331 * t641;
    let t2620 = t331 * t589;
    let t2659 = t562 * t181;
    let t2660 = t2659 * t184;
    let t2677 = t1640 * t219;
    (t2591, t2620, t2659, t2660, t2677)
}

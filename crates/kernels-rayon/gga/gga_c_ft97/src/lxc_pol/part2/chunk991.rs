//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 991/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk991(t2766: f64, t863: f64, t4141: f64, t2681: f64, t309: f64, t1212: f64, t870: f64, t2867: f64, t4147: f64, t8392: f64, t2405: f64, t4150: f64) -> (f64, f64, f64, f64) {
    let t15365 = t2766 * t863;
    let t15366 = t15365 * t4141;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15371 = t15370 * t2867;
    let t15372 = t15369 * t15371;
    let t15376 = 2.0_f64 / 27.0_f64 * t8392 * t4147;
    let t15377 = t4150 * t2405;
    (t15366, t15372, t15376, t15377)
}

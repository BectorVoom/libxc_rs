//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 746/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk746(t1383: f64, t528: f64, t35: f64, t413: f64, t536: f64, t1477: f64, t6: f64, t153: f64, t2704: f64, t2718: f64, t39: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4557 = 0.25128846160651320563e0_f64 * t528 * t1383;
    let t4560 = t35 * t413;
    let t4561 = 24.0_f64 * t4560;
    let t4568 = t536 * t1383;
    let t4573 = t6 * t1477;
    let t4576 = -0.53666666666666666667e-2_f64 * t2704 - 0.60688888888888888888e-1_f64 * t2718 + 0.1829167760955153094e-1_f64 * t39 - 0.36147222222222222223e-2_f64 * t153 * t4573;
    let t4577 = t4576 * t161;
    (t4557, t4560, t4561, t4568, t4573, t4576, t4577)
}

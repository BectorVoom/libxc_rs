//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1218/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1218(t14817: f64, t945: f64, t321: f64, t47184: f64, t50832: f64, t14822: f64, t4188: f64, t6854: f64, t14829: f64, t1172: f64, t1198: f64, t318: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52799 = t14817 * t945;
    let t52801 = 2.0_f64 * t321 * t52799;
    let t52810 = 6.0_f64 * t50832 * t47184;
    let t52812 = 2.0_f64 * t321 * t14822;
    let t52816 = t4188 * t6854;
    let t52821 = 2.0_f64 * t321 * t14829;
    let t52823 = t1172 * t318 * t1198;
    (t52799, t52801, t52810, t52812, t52816, t52821, t52823)
}

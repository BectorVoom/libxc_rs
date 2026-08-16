//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1220/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1220(t14392: f64, t804: f64, t1167: f64, t2074: f64, t1172: f64, t2182: f64, t1105: f64, t2423: f64, t1198: f64, t321: f64, t43260: f64, t14380: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52836 = 6.0_f64 * t804 * t14392;
    let t52837 = t1167 * t2074;
    let t52841 = t1172 * t2182;
    let t52847 = t1105 * t2423;
    let t52853 = 4.0_f64 * t321 * t1198 * t43260;
    let t52855 = 6.0_f64 * t804 * t14380;
    (t52836, t52837, t52841, t52847, t52853, t52855)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1091/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1091(t12213: f64, t2409: f64, t3068: f64, t12072: f64, t328: f64, t2118: f64, t3074: f64, t2246: f64, t3898: f64, t3802: f64, t6472: f64, t11660: f64) -> (f64, f64, f64, f64, f64) {
    let t12215 = t2409 * t12213 * t3068;
    let t12218 = t12072 * t328;
    let t12219 = t2118 * t12218;
    let t12220 = t3074 * t12219;
    let t12223 = t2246 * t3898;
    let t12227 = t3802 * t328;
    let t12228 = t6472 * t12227;
    let t12229 = t11660 * t12228;
    (t12215, t12220, t12223, t12227, t12229)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 827/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk827(t810: f64, t898: f64, t938: f64, t353: f64, t4386: f64, t2239: f64, t2246: f64, t329: f64, t369: f64, t838: f64, t2404: f64, t2052: f64, t381: f64) -> (f64, f64, f64, f64) {
    let t6794 = t898 * t810;
    let t6795 = t6794 * t938;
    let t6796 = t353 * t6795;
    let t6797 = t4386 * t6796;
    let t6805 = t2246 * t2239;
    let t6832 = t329 * t838 * t369;
    let t6833 = t6832 * t2404;
    let t6854 = 1.0_f64 / t2052 / t381;
    (t6797, t6805, t6833, t6854)
}

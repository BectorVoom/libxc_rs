//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 517/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk517(t2195: f64, t858: f64, t867: f64, t866: f64, t1477: f64, t56: f64) -> (f64, f64, f64, f64) {
    let t2196 = t858 * t2195;
    let t2197 = t867 * t2196;
    let t2199 = t866 * t2197 / 96.0_f64;
    let t2200 = t1477 * t56;
    (t2196, t2197, t2199, t2200)
}

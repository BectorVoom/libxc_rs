//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 789/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk789(t5931: f64, t670: f64, t1: f64, t1354: f64, t3: f64, t672: f64, t1996: f64, t2000: f64, t2007: f64, t671: f64, t703: f64, t2003: f64) -> (f64, f64, f64, f64, f64) {
    let t5933 = 0.21642082724729686754e0_f64 * t670 * t5931;
    let t5935 = t1354 * t1 * t3;
    let t5936 = t5935 * t672;
    let t5938 = t1996 * t2000;
    let t5940 = t1996 * t2007;
    let t5942 = t703 * t671;
    let t5944 = 0.11181742741110338156e-1_f64 * t2003 * t5942;
    (t5933, t5936, t5938, t5940, t5944)
}

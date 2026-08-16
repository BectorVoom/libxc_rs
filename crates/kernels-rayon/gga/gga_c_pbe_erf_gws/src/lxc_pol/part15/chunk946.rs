//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 946/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk946(t2007: f64, t2970: f64, t1: f64, t2522: f64, t3: f64, t672: f64, t2000: f64, t7038: f64, t7042: f64, t7045: f64, t7047: f64, t7054: f64, t7060: f64, t7067: f64, t7072: f64, t7077: f64, t7079: f64, t7080: f64, t7083: f64, t7084: f64) -> f64 {
    let t8408 = t2970 * t2007;
    let t8411 = t2522 * t1 * t3;
    let t8413 = 0.21642082724729686754e0_f64 * t8411 * t672;
    let t8414 = t2970 * t2000;
    let t8416 = -t7038 + t7042 - t7045 + t7047 + 0.72140275749098955847e-1_f64 * t8408 + t8413 + 0.21642082724729686754e0_f64 * t8414 + t7054 - t7060 + t7067 - t7072 + t7077 - t7079 - t7080 - t7083 - t7084;
    t8416
}

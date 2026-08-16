//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 529/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk529(t274: f64, t343: f64, t874: f64, t851: f64, t2255: f64, t359: f64, t362: f64, t366: f64, t899: f64, t2158: f64, t904: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2257 = t274 * t874 * t343;
    let t2258 = t851 * t2257;
    let t2259 = t2255 * t2258;
    let t2262 = t359 * t359;
    let t2263 = 1.0_f64 / t2262;
    let t2264 = t2263 * t362;
    let t2266 = t899 * t2264 * t366;
    let t2268 = t916 * t904 * t2158;
    (t2257, t2258, t2259, t2262, t2263, t2264, t2266, t2268)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 621/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk621(t1049: f64, t679: f64, t1988: f64, t1992: f64, t1997: f64, t2002: f64, t2006: f64, t231: f64, t2534: f64, t2535: f64, t2558: f64, t2564: f64, t2569: f64, t2574: f64, t2578: f64, t2583: f64, t2587: f64, t2960: f64, t2962: f64) -> f64 {
    let t2965 = t1049 * t679;
    let t2968 = t1988 + 4.0_f64 / 3.0_f64 * t1992 + 4.0_f64 / 3.0_f64 * t2960 + t2534 + t2535 - t2558 + t2564 - t2569 + t2574 + t2578 + t2583 + t2587 + 4.0_f64 / 3.0_f64 * t2962 * t231 + 4.0_f64 / 3.0_f64 * t2965 + 0.10821041362364843377e0_f64 * t1997 + t2002 + t2006;
    t2968
}

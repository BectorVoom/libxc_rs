//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 476/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk476(t1: f64, t3: f64, t535: f64, t672: f64, t225: f64, t677: f64, t10: f64, t670: f64, t20: f64, t711: f64, t245: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1996 = t535 * t1 * t3;
    let t1997 = t1996 * t672;
    let t1999 = t225 * t677;
    let t2000 = t10 * t1999;
    let t2002 = 0.21642082724729686754e0_f64 * t670 * t2000;
    let t2003 = t711 * t20;
    let t2004 = t245 * t671;
    (t1996, t1997, t1999, t2000, t2002, t2003, t2004)
}

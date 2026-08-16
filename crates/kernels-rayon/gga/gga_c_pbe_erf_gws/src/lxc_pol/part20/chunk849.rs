//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 849/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk849(t1049: f64, t1986: f64, t2007: f64, t2970: f64, t1: f64, t2522: f64, t3: f64, t672: f64, t2000: f64, t20: f64, t2653: f64, t2004: f64) -> (f64, f64, f64, f64, f64) {
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    let t8411 = t2522 * t1 * t3;
    let t8413 = 0.21642082724729686754e0_f64 * t8411 * t672;
    let t8414 = t2970 * t2000;
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    (t8405, t8408, t8413, t8414, t8425)
}

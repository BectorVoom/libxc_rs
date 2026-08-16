//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 671/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk671(t1267: f64, t1271: f64, t1394: f64, t1398: f64, t1431: f64, t1446: f64, t2064: f64, t2098: f64, t3365: f64, t3366: f64, t3367: f64, t3368: f64, t3370: f64, t3371: f64) -> f64 {
    let t3771 = -t2064 - t3365 - t1431 + t3370 - t1271 + t1446 + t3371 - t1267 + t2098 - t1394 - t1398 - t3366 - t3367 - t3368;
    t3771
}

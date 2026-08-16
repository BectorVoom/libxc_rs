//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1002/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1002(t17432: f64, t17434: f64, t17436: f64, t17439: f64, t17443: f64, t17448: f64, t17450: f64, t17452: f64, t17456: f64, t17461: f64, t17463: f64, t17465: f64) -> f64 {
    let t18282 = t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452 - t17456 - t17461 + t17463 + t17465;
    t18282
}

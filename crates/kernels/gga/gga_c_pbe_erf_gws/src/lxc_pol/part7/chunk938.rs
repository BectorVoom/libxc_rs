//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 938/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk938<F: Float>(t1630: F, t5224: F, t639: F, t16973: F, t5003: F, t642: F, t17456: F, t17461: F, t17463: F, t17465: F, t17467: F, t17469: F, t17473: F, t17476: F, t17481: F) -> (F, F, F) {
    let t17483 = t639 * t1630 * t5224;
    let t17484 = F::new(64.0) / F::new(45.0) * t17483;
    let t17488 = F::new(32.0) / F::new(15.0) * t639 * t642 * t5003 * t16973;
    let t17489 = -t17456 - t17461 + t17463 + t17465 + t17467 - t17469 + t17473 - t17476 - t17481 + t17484 - t17488;
    (t17484, t17488, t17489)
}

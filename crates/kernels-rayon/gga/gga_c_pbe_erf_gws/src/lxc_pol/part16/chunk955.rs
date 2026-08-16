//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 955/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk955(t5359: f64, t7601: f64, t7603: f64, t7605: f64, t7607: f64, t7609: f64, t7613: f64, t7615: f64, t7617: f64, t7619: f64, t7621: f64, t7623: f64, t7625: f64, t7629: f64, t7634: f64, t7636: f64, t7637: f64) -> f64 {
    let t8443 = t7601 + t7603 + t7605 + t7607 + t7609 - t7613 + t7615 + t7617 + t7619 + t7621 + t7623 - t7625 + t7629 + t7634 - t7636 - t7637 + t5359;
    t8443
}

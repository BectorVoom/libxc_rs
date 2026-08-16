//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1003/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1003(t11106: f64, t11108: f64, t11109: f64, t11114: f64, t11118: f64, t11120: f64, t11122: f64, t11124: f64, t11128: f64, t11130: f64, t11135: f64, t11140: f64, t11142: f64, t11144: f64, t11146: f64, t7968: f64, t7970: f64) -> f64 {
    let t11236 = t11106 - t11108 + t11109 + t11114 + t11118 + t11120 - t11122 + t11124 + t11128 + t11130 + t11135 - t11140 + t7968 + t7970 - t11142 - t11144 + t11146;
    t11236
}

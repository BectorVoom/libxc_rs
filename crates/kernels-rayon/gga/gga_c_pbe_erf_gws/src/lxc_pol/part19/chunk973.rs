//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 973/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk973(t10995: f64, t5218: f64, t7148: f64, t995: f64, t2555: f64, t7811: f64, t10959: f64, t10963: f64, t10967: f64, t10971: f64, t10974: f64, t10975: f64, t10977: f64, t10981: f64, t10984: f64, t10987: f64, t10991: f64, t10994: f64, t5521: f64, t7810: f64) -> (f64, f64, f64, f64) {
    let t10997 = 16.0_f64 / 45.0_f64 * t5218 * t10995;
    let t10998 = t7148 * t995;
    let t10999 = t10998 * t2555;
    let t11001 = 32.0_f64 / 45.0_f64 * t5218 * t10999;
    let t11002 = 8.0_f64 / 135.0_f64 * t7811;
    let t11003 = -t10959 - t10963 - t10967 + t10971 + t10974 - t10975 - t7810 - t5521 - t10977 + t10981 + t10984 - t10987 + t10991 - t10994 - t10997 - t11001 - t11002;
    (t10997, t11001, t11002, t11003)
}

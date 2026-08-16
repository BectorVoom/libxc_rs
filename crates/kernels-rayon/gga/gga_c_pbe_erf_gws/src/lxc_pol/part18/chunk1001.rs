//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1001/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1001(t10971: f64, t10974: f64, t10975: f64, t10977: f64, t10981: f64, t10984: f64, t10987: f64, t10991: f64, t10994: f64, t10997: f64, t11001: f64, t11002: f64, t11004: f64, t7810: f64, t7852: f64, t7870: f64) -> f64 {
    let t11225 = t10971 + t10974 - t10975 - t7810 - t10977 + t10981 + t10984 - t10987 + t10991 - t10994 - t10997 - t11001 - t11002 + t11004 + t7852 + t7870;
    t11225
}

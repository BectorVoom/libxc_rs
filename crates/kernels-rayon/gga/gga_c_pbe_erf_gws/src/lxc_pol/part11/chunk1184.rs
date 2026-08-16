//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1184/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1184(t47910: f64, t47914: f64, t47916: f64, t47918: f64, t47920: f64, t47922: f64, t47926: f64, t47928: f64, t48043: f64, t48044: f64, t48045: f64, t26308: f64, t26314: f64, t41334: f64, t48046: f64, t48049: f64, t48050: f64, t48052: f64, t48056: f64, t48059: f64, t48060: f64, t48062: f64) -> (f64, f64) {
    let t48659 = -t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926 + t47928 - t48043 - t48044 + t48045;
    let t48663 = -t48046 + t48049 - t48050 - t48052 - t48056 + 0.44134814814814814813e-2_f64 * t26308 + 16.0_f64 * t26314 + t48059 + t48060 + 0.43284165449459373508e0_f64 * t41334 - t48062;
    (t48659, t48663)
}

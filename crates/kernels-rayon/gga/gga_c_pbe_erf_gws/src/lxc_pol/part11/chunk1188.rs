//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1188/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1188(t48203: f64, t48207: f64, t48213: f64, t48215: f64, t48219: f64, t48223: f64, t48225: f64, t48227: f64, t48229: f64, t48231: f64, t48232: f64, t48233: f64, t48234: f64, t48261: f64, t48265: f64, t48267: f64, t48270: f64, t48272: f64, t48274: f64, t48275: f64, t48279: f64, t48282: f64, t48285: f64) -> (f64, f64) {
    let t48681 = t48203 - t48207 - t48213 + t48215 + t48219 + t48223 - t48225 + t48227 + t48229 - t48231 + t48232;
    let t48682 = t48233 + t48234 + t48261 + t48265 - t48267 + t48270 + t48272 + t48274 - t48275 - t48279 - t48282 + t48285;
    (t48681, t48682)
}

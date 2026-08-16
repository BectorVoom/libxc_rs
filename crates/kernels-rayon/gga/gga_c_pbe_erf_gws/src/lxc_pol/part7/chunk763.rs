//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 763/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk763(t2147: f64, t6257: f64, t2146: f64, t2271: f64, t2337: f64, t905: f64, t6110: f64, t821: f64, t2189: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t6258 = t2147 * t6257;
    let t6260 = t2146 * t6258 / 16.0_f64;
    let t6261 = t2337 * t2271;
    let t6262 = t905 * t6261;
    let t6265 = t821 * t6110;
    let t6266 = t905 * t6265;
    let t6269 = t5 * t2189;
    (t6258, t6260, t6262, t6266, t6269)
}

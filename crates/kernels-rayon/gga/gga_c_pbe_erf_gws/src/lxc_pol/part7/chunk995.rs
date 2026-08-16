//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 995/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk995(t17219: f64, t17222: f64, t17225: f64, t17229: f64, t17232: f64, t17234: f64, t17237: f64, t17239: f64, t17246: f64, t17251: f64, t17255: f64, t17257: f64, t17259: f64, t17264: f64, t17267: f64, t17271: f64, t17275: f64, t17279: f64, t17282: f64, t17285: f64, t17287: f64, t17291: f64, t17293: f64) -> (f64, f64) {
    let t18228 = -t17219 + t17222 - t17225 + t17229 - t17232 + t17234 + t17237 + t17239 + t17246 + t17251 + t17255;
    let t18229 = -t17257 + t17259 - t17264 + t17267 - t17271 + t17275 - t17279 - t17282 + t17285 + t17287 + t17291 - t17293;
    (t18228, t18229)
}

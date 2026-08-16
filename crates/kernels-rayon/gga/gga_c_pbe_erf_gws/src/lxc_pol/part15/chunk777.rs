//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 777/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk777(t39: f64, t535: f64, t159: f64, t285: f64, t169: f64, t301: f64, t366: f64, t745: f64, t1354: f64, t532: f64, t1500: f64, t2036: f64) -> (f64, f64, f64, f64) {
    let t5668 = t39 * t535;
    let t5670 = t5668 * t159 * t285;
    let t5674 = t169 * t366 * t745 * t301;
    let t5676 = t532 * t1354;
    let t5678 = t5676 * t159 * t285;
    let t5680 = t1500 * t2036;
    (t5670, t5674, t5678, t5680)
}

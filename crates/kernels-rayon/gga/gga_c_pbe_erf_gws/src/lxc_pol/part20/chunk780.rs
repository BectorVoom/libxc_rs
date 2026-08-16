//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 780/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk780(t2030: f64, t520: f64, t39: f64, t535: f64, t159: f64, t285: f64, t169: f64, t301: f64, t366: f64, t745: f64, t1457: f64, t545: f64) -> (f64, f64, f64, f64) {
    let t5660 = t2030 * t520;
    let t5668 = t39 * t535;
    let t5670 = t5668 * t159 * t285;
    let t5674 = t169 * t366 * t745 * t301;
    let t5690 = t1457 * t545 * t285;
    (t5660, t5670, t5674, t5690)
}

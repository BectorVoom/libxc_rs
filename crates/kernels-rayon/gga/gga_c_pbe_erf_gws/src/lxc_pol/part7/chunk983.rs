//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 983/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk983(t16609: f64, t16611: f64, t16616: f64, t16620: f64, t16624: f64, t16630: f64, t16633: f64, t16636: f64, t16639: f64, t16642: f64, t16645: f64, t5202: f64, t735: f64) -> (f64, f64) {
    let t18159 = t16609 - t16611 + t16616 + t16620 + t16624 + t16630 - t16633 - t16636 - t16639 + t16642 + t16645;
    let t18160 = t5202 * t735;
    (t18159, t18160)
}

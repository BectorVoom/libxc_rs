//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 778/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk778(t1365: f64, t153: f64, t745: f64, t1464: f64, t242: f64, t366: f64, t5: f64, t168: f64, t270: f64, t274: f64, t4573: f64, t1503: f64, t522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5585 = t153 * t1365 * t745;
    let t5588 = 0.50257692321302641125e0_f64 * t1464 * t242;
    let t5589 = t5 * t366;
    let t5592 = 0.19455129084526283664e0_f64 * t168 * t5589 * t270;
    let t5595 = 0.4429070076315393047e1_f64 * t153 * t4573 * t274;
    let t5601 = t1503 * t522;
    (t5585, t5588, t5589, t5592, t5595, t5601)
}

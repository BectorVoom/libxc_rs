//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 261/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk261(t159: f64, t285: f64, t762: f64, t147: f64, t299: f64, t169: f64, t242: f64, t171: f64, t535: f64, t289: f64, t700: f64, t274: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t765 = 0.29056741517886919367e-3_f64 * t762 * t159 * t285;
    let t766 = t299 * t147;
    let t769 = 0.53059442957798955452e-1_f64 * t169 * t766 * t242;
    let t770 = t171 * t535;
    let t776 = 0.31835665774679373271e-1_f64 * t169 * t289 * t700;
    let t778 = 0.1066501354843587606e0_f64 * t532 * t274;
    (t765, t766, t769, t770, t776, t778)
}

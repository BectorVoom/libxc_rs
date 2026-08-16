//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 249/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk249<F: Float>(t159: F, t285: F, t762: F, t147: F, t299: F, t169: F, t242: F, t171: F, t535: F, t289: F, t700: F, t274: F, t532: F) -> (F, F, F, F, F, F) {
    let t765 = F::cast_from(0.29056741517886919367e-3_f64) * t762 * t159 * t285;
    let t766 = t299 * t147;
    let t769 = F::cast_from(0.53059442957798955452e-1_f64) * t169 * t766 * t242;
    let t770 = t171 * t535;
    let t776 = F::cast_from(0.31835665774679373271e-1_f64) * t169 * t289 * t700;
    let t778 = F::cast_from(0.1066501354843587606e0_f64) * t532 * t274;
    (t765, t766, t769, t770, t776, t778)
}

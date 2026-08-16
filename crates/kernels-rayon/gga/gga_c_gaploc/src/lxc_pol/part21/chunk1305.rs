//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1305/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1305(t10524: f64, t10527: f64, t1397: f64, t10314: f64, t2476: f64, t580: f64, t2877: f64, t30292: f64, t2375: f64, t26451: f64, t26455: f64, t6904: f64, t8248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34294 = 0.42900587942220512002e1_f64 * t1397 * t10524 * t10527;
    let t34297 = 0.12269736305254639897e2_f64 * t2476 * t580 * t10314;
    let t34299 = 0.35750489951850426669e0_f64 * t30292 * t2877;
    let t34301 = 0.23833659967900284446e0_f64 * t26451 * t2375;
    let t34303 = 0.23833659967900284446e0_f64 * t26455 * t2375;
    let t34305 = 0.23833659967900284446e0_f64 * t8248 * t6904;
    (t34294, t34297, t34299, t34301, t34303, t34305)
}

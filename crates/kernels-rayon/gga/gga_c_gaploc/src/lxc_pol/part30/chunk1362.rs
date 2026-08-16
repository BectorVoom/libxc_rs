//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1362/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1362(t2877: f64, t30292: f64, t2375: f64, t26451: f64, t26455: f64, t6904: f64, t8248: f64, t26763: f64, t7030: f64, t2389: f64, t8229: f64, t8331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34299 = 0.35750489951850426669e0_f64 * t30292 * t2877;
    let t34301 = 0.23833659967900284446e0_f64 * t26451 * t2375;
    let t34303 = 0.23833659967900284446e0_f64 * t26455 * t2375;
    let t34305 = 0.23833659967900284446e0_f64 * t8248 * t6904;
    let t34306 = t26763 * t7030;
    let t34307 = 0.29792074959875355558e-1_f64 * t34306;
    let t34308 = t8229 * t2389;
    let t34309 = 0.59584149919750711116e-1_f64 * t34308;
    let t34310 = t8331 * t2389;
    (t34299, t34301, t34303, t34305, t34307, t34309, t34310)
}

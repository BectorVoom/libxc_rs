//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1365/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1365<F: Float>(t2877: F, t30292: F, t2375: F, t26451: F, t26455: F, t6904: F, t8248: F, t26763: F, t7030: F, t2389: F, t8229: F, t8331: F) -> (F, F, F, F, F, F, F) {
    let t34299 = F::cast_from(0.35750489951850426669e0_f64) * t30292 * t2877;
    let t34301 = F::cast_from(0.23833659967900284446e0_f64) * t26451 * t2375;
    let t34303 = F::cast_from(0.23833659967900284446e0_f64) * t26455 * t2375;
    let t34305 = F::cast_from(0.23833659967900284446e0_f64) * t8248 * t6904;
    let t34306 = t26763 * t7030;
    let t34307 = F::cast_from(0.29792074959875355558e-1_f64) * t34306;
    let t34308 = t8229 * t2389;
    let t34309 = F::cast_from(0.59584149919750711116e-1_f64) * t34308;
    let t34310 = t8331 * t2389;
    (t34299, t34301, t34303, t34305, t34307, t34309, t34310)
}

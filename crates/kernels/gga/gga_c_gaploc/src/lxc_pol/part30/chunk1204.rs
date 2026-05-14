//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1204/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1204<F: Float>(t10524: F, t10527: F, t1397: F, t10314: F, t2476: F, t580: F, t2877: F, t30292: F, t2375: F, t26451: F, t26455: F, t6904: F, t8248: F, t26763: F, t7030: F, t2389: F, t8229: F) -> (F, F, F, F, F, F, F, F) {
    let t34294 = 0.42900587942220512002e1 * t1397 * t10524 * t10527;
    let t34297 = 0.12269736305254639897e2 * t2476 * t580 * t10314;
    let t34299 = 0.35750489951850426669e0 * t30292 * t2877;
    let t34301 = 0.23833659967900284446e0 * t26451 * t2375;
    let t34303 = 0.23833659967900284446e0 * t26455 * t2375;
    let t34305 = 0.23833659967900284446e0 * t8248 * t6904;
    let t34306 = t26763 * t7030;
    let t34307 = 0.29792074959875355558e-1 * t34306;
    let t34308 = t8229 * t2389;
    (t34294, t34297, t34299, t34301, t34303, t34305, t34307, t34308)
}

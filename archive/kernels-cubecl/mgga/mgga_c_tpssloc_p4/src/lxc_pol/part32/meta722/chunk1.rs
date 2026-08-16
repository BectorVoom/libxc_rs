//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2300/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300<F: Float>(t225: F, t29685: F, t103218: F, t1238: F, t1252: F, t19208: F, t19232: F, t19234: F, t2154: F, t24633: F, t27406: F, t27747: F, t27752: F, t27794: F, t27812: F, t29798: F, t29812: F, t3593: F, t3598: F, t5055: F, t5088: F, t7283: F, t7291: F, t7356: F, t7392: F, t8087: F, t94700: F, t94701: F) -> F {
    let t103464 = t29685 * t225;
    let t103488 = F::cast_from(4.0_f64) * t5055 * t27747 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27752 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27812 - t103464 * t1252 + F::cast_from(4.0_f64) * t1238 * t3598 * t8087 * t5088 - F::cast_from(6.0_f64) * t3593 * t29798 + F::cast_from(2.0_f64) * t19232 * t7356 - F::cast_from(0.80418998823691070228e-1_f64) * t103218 * t7291 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27794 + F::cast_from(2.0_f64) * t1238 * t3598 * t2154 * t19208 - t94700 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t24633 * t29812 + F::cast_from(0.36554090374405031923e-2_f64) * t94701 - F::cast_from(2.0_f64) * t19234 * t7392;
    t103488
}

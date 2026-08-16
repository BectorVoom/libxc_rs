//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2300/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2300(t225: f64, t29685: f64, t103218: f64, t1238: f64, t1252: f64, t19208: f64, t19232: f64, t19234: f64, t2154: f64, t24633: f64, t27406: f64, t27747: f64, t27752: f64, t27794: f64, t27812: f64, t29798: f64, t29812: f64, t3593: f64, t3598: f64, t5055: f64, t5088: f64, t7283: f64, t7291: f64, t7356: f64, t7392: f64, t8087: f64, t94700: f64, t94701: f64) -> f64 {
    let t103464 = t29685 * t225;
    let t103488 = 4.0_f64 * t5055 * t27747 + 0.43864908449286038306e-1_f64 * t27406 * t27752 + 0.43864908449286038306e-1_f64 * t27406 * t27812 - t103464 * t1252 + 4.0_f64 * t1238 * t3598 * t8087 * t5088 - 6.0_f64 * t3593 * t29798 + 2.0_f64 * t19232 * t7356 - 0.80418998823691070228e-1_f64 * t103218 * t7291 + 0.43864908449286038306e-1_f64 * t27406 * t27794 + 2.0_f64 * t1238 * t3598 * t2154 * t19208 - t94700 - 0.27415567780803773942e-2_f64 * t7283 * t24633 * t29812 + 0.36554090374405031923e-2_f64 * t94701 - 2.0_f64 * t19234 * t7392;
    t103488
}

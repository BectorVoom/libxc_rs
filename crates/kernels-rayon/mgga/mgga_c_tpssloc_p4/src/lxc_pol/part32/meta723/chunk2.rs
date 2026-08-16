//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2308/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2308(t6260: f64, t7327: f64, t24660: f64, t6252: f64, t1215: f64, t5392: f64, t7376: f64, t27736: f64, t7999: f64, t103218: f64, t11904: f64, t24849: f64, t27406: f64, t27455: f64, t27525: f64, t27532: f64, t27733: f64, t29678: f64, t29719: f64, t29723: f64, t3610: f64, t4930: f64, t5068: f64, t7283: f64, t7365: f64, t7382: f64, t8077: f64, t86037: f64, t86039: f64, t86076: f64, t86077: f64, t94837: f64, t95048: f64) -> (f64, f64) {
    let t103767 = t7327 * t6260;
    let t103774 = t24660 * t6252;
    let t103779 = t5392 * t1215 * t7376;
    let t103799 = t7999 * t27736;
    let t103801 = -0.27415567780803773942e-2_f64 * t24849 * t103767 * t27532 - 0.54831135561607547884e-2_f64 * t24849 * t94837 * t27525 - 0.54831135561607547883e-2_f64 * t86037 * t103774 * t86039 + 0.36554090374405031923e-2_f64 * t86076 * t86077 * t103779 + 2.0_f64 * t11904 * t29723 - 0.43864908449286038306e-1_f64 * t7999 * t27733 - t95048 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t8077 + 0.43864908449286038306e-1_f64 * t27406 * t27455 - 0.26806332941230356743e-1_f64 * t103218 * t7365 + 4.0_f64 * t3610 * t29719 * t5068 + 0.80418998823691070228e-1_f64 * t29678 * t7382 - 0.14621636149762012769e-1_f64 * t103799;
    (t103779, t103801)
}

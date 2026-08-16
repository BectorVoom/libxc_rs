//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 967/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk967(t30066: f64, t30082: f64, t30099: f64, t30115: f64, t12352: f64, t18925: f64, t2042: f64, t25153: f64, t2666: f64, t29350: f64, t29352: f64, t29354: f64, t29356: f64, t29359: f64, t29362: f64, t29628: f64, t30037: f64, t30045: f64, t30048: f64, t5532: f64, t7656: f64, t802: f64, t9262: f64, t9291: f64) -> f64 {
    let t30117 = t30066 + t30082 + t30099 + t30115;
    let t30119 = -6.0_f64 * t12352 * t30045 + 6.0_f64 * t18925 * t9262 - t2042 * t30117 - 3.0_f64 * t25153 * t2666 + t30037 * t802 + 6.0_f64 * t30048 * t5532 - 3.0_f64 * t7656 * t9291 - t29350 + t29352 - t29354 + t29356 + t29359 - t29362 + t29628;
    t30119
}

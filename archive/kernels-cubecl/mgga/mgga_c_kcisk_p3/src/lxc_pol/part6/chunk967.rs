//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 967/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk967<F: Float>(t30066: F, t30082: F, t30099: F, t30115: F, t12352: F, t18925: F, t2042: F, t25153: F, t2666: F, t29350: F, t29352: F, t29354: F, t29356: F, t29359: F, t29362: F, t29628: F, t30037: F, t30045: F, t30048: F, t5532: F, t7656: F, t802: F, t9262: F, t9291: F) -> F {
    let t30117 = t30066 + t30082 + t30099 + t30115;
    let t30119 = -F::cast_from(6.0_f64) * t12352 * t30045 + F::cast_from(6.0_f64) * t18925 * t9262 - t2042 * t30117 - F::cast_from(3.0_f64) * t25153 * t2666 + t30037 * t802 + F::cast_from(6.0_f64) * t30048 * t5532 - F::cast_from(3.0_f64) * t7656 * t9291 - t29350 + t29352 - t29354 + t29356 + t29359 - t29362 + t29628;
    t30119
}

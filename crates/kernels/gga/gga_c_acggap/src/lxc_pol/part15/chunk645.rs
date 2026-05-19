//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 645/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk645<F: Float>(t301: F, t6308: F, t960: F, t1879: F, t372: F, t1150: F, t1173: F, t335: F, t367: F, t3671: F, t3673: F, t3677: F, t3679: F, t3694: F, t3699: F, t3702: F, t5169: F, t5175: F, t6271: F, t6279: F, t6283: F, t6286: F, t6290: F, t6294: F, t6297: F, t6301: F, t6305: F) -> (F, F, F) {
    let t6309 = t6308 * t301;
    let t6310 = t960 * t6309;
    let t6313 = t1879 * t372;
    let t6314 = t960 * t6313;
    let t6317 = F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t6271 - t5169 - t5175 - F::cast_from(0.22675591804667994221e-1_f64) * t3671 + F::cast_from(0.11337795902333997111e-1_f64) * t3673 - F::cast_from(0.11337795902333997111e-1_f64) * t3677 + F::cast_from(0.80031500487063509016e-2_f64) * t3679 - t3694 - t3699 - t3702 + t367 * t6279 / F::new(48.0) + t1150 * t6283 / F::new(16.0) - F::new(7.0) / F::new(48.0) * t6286 + t335 * t6290 / F::new(48.0) + t367 * t6294 / F::new(48.0) + t335 * t6297 / F::new(24.0) + t335 * t6301 / F::new(24.0) + t335 * t6305 / F::new(24.0) - t335 * t6310 / F::new(24.0) - t367 * t6314 / F::new(16.0);
    (t6309, t6313, t6317)
}

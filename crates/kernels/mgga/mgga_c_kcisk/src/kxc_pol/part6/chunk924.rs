//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 924/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk924<F: Float>(t2571: F, t9054: F, t734: F, t24203: F, t2564: F, t17976: F, t9072: F, t29363: F, t29365: F, t29368: F, t29370: F, t29373: F, t29376: F, t29378: F, t29380: F, t29383: F, t29386: F, t29390: F, t29393: F) -> (F, F, F, F) {
    let t29395 = t9054 * t2571;
    let t29396 = t734 * t29395;
    let t29398 = t24203 * t2564;
    let t29400 = t17976 * t9072;
    let t29402 = -t29363 / F::cast_from(2.0_f64) - F::cast_from(19.0_f64) / F::cast_from(48.0_f64) * t29365 - t29368 / F::cast_from(8.0_f64) + t29370 / F::cast_from(16.0_f64) + t29373 / F::cast_from(24.0_f64) - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t29376 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29378 + t29380 / F::cast_from(32.0_f64) - t29383 / F::cast_from(8.0_f64) - t29386 / F::cast_from(32.0_f64) - F::cast_from(77.0_f64) / F::cast_from(27.0_f64) * t29390 + t29393 / F::cast_from(64.0_f64) + F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t29396 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t29398 - t29400 / F::cast_from(6.0_f64);
    (t29396, t29398, t29400, t29402)
}

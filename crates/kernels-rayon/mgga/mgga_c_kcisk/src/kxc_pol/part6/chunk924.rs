//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 924/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk924(t2571: f64, t9054: f64, t734: f64, t24203: f64, t2564: f64, t17976: f64, t9072: f64, t29363: f64, t29365: f64, t29368: f64, t29370: f64, t29373: f64, t29376: f64, t29378: f64, t29380: f64, t29383: f64, t29386: f64, t29390: f64, t29393: f64) -> (f64, f64, f64, f64) {
    let t29395 = t9054 * t2571;
    let t29396 = t734 * t29395;
    let t29398 = t24203 * t2564;
    let t29400 = t17976 * t9072;
    let t29402 = -t29363 / 2.0_f64 - 19.0_f64 / 48.0_f64 * t29365 - t29368 / 8.0_f64 + t29370 / 16.0_f64 + t29373 / 24.0_f64 - 3.0_f64 / 128.0_f64 * t29376 + 2.0_f64 / 3.0_f64 * t29378 + t29380 / 32.0_f64 - t29383 / 8.0_f64 - t29386 / 32.0_f64 - 77.0_f64 / 27.0_f64 * t29390 + t29393 / 64.0_f64 + 11.0_f64 / 9.0_f64 * t29396 - 3.0_f64 / 16.0_f64 * t29398 - t29400 / 6.0_f64;
    (t29396, t29398, t29400, t29402)
}

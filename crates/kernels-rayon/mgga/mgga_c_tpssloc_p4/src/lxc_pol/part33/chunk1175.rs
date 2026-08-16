//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1175/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1175(t1512: f64, t25146: f64, t5614: f64, t6614: f64, t5617: f64, t815: f64, t6605: f64, t2628: f64, t5585: f64, t23096: f64, t23106: f64, t23108: f64, t25065: f64, t26619: f64, t26621: f64, t28357: f64, t28360: f64, t28362: f64, t28364: f64, t28366: f64) -> (f64, f64, f64) {
    let t28368 = t25146 * t1512;
    let t28370 = t6614 * t5614;
    let t28372 = t815 * t5617;
    let t28373 = t6605 * t28372;
    let t28375 = t2628 * t5585;
    let t28376 = t6605 * t28375;
    let t28378 = 0.40372756094140390854e-3_f64 * t25065 - 0.20186378047070195427e-3_f64 * t28357 + t28360 / 1536.0_f64 - t28362 / 384.0_f64 + t26619 - t26621 - t28364 / 1536.0_f64 + t28366 / 768.0_f64 - t28368 / 768.0_f64 - t28370 / 1536.0_f64 + t23096 - t23106 - 0.20186378047070195427e-3_f64 * t28373 + 0.40372756094140390854e-3_f64 * t28376 + t23108;
    (t28372, t28375, t28378)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2134/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134(t10470: f64, t11058: f64, t381: f64, t1615: f64, t6739: f64, t11064: f64, t3199: f64, t49649: f64, t11045: f64, t10164: f64, t1634: f64, t11190: f64, t1670: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50508 = t10470 * t11058 * t381;
    let t50509 = t1615 * t6739;
    let t50516 = t10470 * t11064 * t381;
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50628 = t10164 * t1634;
    let t50819 = t11190 * t1670;
    (t50508, t50509, t50516, t50592, t50610, t50628, t50819)
}

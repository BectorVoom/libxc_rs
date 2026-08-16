//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2563/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2563(t13950: f64, t3117: f64, t14202: f64, t3048: f64, t14206: f64, t3108: f64, t3185: f64, t49649: f64, t10470: f64, t11058: f64, t381: f64, t1615: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50438 = t3117 * t13950;
    let t50442 = t3048 * t14202;
    let t50445 = t14206 * t3108;
    let t50465 = t49649 * t3185;
    let t50508 = t10470 * t11058 * t381;
    let t50509 = t1615 * t6739;
    (t50438, t50442, t50445, t50465, t50508, t50509)
}

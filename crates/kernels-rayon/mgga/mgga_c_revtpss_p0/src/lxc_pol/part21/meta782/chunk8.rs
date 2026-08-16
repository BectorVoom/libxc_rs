//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2808/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2808(t1558: f64, t2482: f64, t2801: f64, t2815: f64, t10547: f64, t14606: f64, t10538: f64, t14605: f64, t49180: f64, t14586: f64, t2645: f64, t10529: f64, t2782: f64) -> (f64, f64, f64, f64) {
    let t51598 = t2482 * t2815 * t1558 * t2801;
    let t51600 = t14606 * t10547;
    let t51603 = t49180 * t14605 * t10538;
    let t51604 = 0.34697458558045176417e-2_f64 * t51603;
    let t51608 = t14586 * t2645;
    let t51610 = t2782 * t10529 * t51608;
    (t51598, t51600, t51604, t51610)
}

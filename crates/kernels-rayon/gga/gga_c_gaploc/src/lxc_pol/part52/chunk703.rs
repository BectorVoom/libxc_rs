//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 703/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk703(t13625: f64, t969: f64, t825: f64, t123: f64, t3614: f64, t883: f64, t2685: f64, t2684: f64, t7428: f64, t7427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13626 = t969 * t13625;
    let t13627 = t825 * t13626;
    let t13630 = t3614 * t123;
    let t13631 = t13630 * t883;
    let t13632 = t2685 * t13631;
    let t13633 = t2684 * t13632;
    let t13634 = 0.19171462976960374838e0_f64 * t13633;
    let t13635 = t7428 * t13625;
    let t13636 = t7427 * t13635;
    let t13638 = t2685 * t13625;
    let t13639 = t2684 * t13638;
    let t13641 = t969 * t13631;
    let t13642 = t825 * t13641;
    (t13626, t13627, t13630, t13631, t13632, t13634, t13635, t13636, t13638, t13639, t13641, t13642)
}

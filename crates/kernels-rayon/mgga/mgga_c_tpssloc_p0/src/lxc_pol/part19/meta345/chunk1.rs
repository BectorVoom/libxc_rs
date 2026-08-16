//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1235/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1235(t2678: f64, t2632: f64, t2681: f64, t9671: f64, t2628: f64, t2690: f64, t812: f64, t2635: f64, t232: f64, t40925: f64, t2379: f64, t2553: f64, t2630: f64, t2686: f64, t40934: f64, t40938: f64, t41344: f64, t41349: f64, t41355: f64, t41363: f64, t41365: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9613: f64, t9967: f64, t9974: f64, t9978: f64, t9983: f64) -> (f64, f64, f64, f64) {
    let t41367 = t2678 * t2678;
    let t41368 = t41367 * t2632;
    let t41373 = t9671 * t2681;
    let t41385 = t812 * t2628 * t2690;
    let t41386 = t41385 * t2635;
    let t41388 = t40925 * t232;
    let t41393 = -t41344 * t9978 / 128.0_f64 + t41349 * t819 * t820 * t40934 / 128.0_f64 + 7.0_f64 / 384.0_f64 * t41355 - 3.0_f64 / 256.0_f64 * t9974 * t819 * t820 * t40938 + 595.0_f64 / 2592.0_f64 * t41363 - 119.0_f64 / 2304.0_f64 * t41365 + t2630 * t819 * t820 * t41368 / 512.0_f64 - 119.0_f64 / 2304.0_f64 * t41373 - 15.0_f64 / 64.0_f64 * t843 * t9607 * t820 * t2379 * t2553 + t9967 * t9983 / 128.0_f64 - t9613 * t2686 / 512.0_f64 + 119.0_f64 / 1152.0_f64 * t41386 - t817 * t819 * t820 * t41388 / 3072.0_f64;
    (t41367, t41368, t41388, t41393)
}

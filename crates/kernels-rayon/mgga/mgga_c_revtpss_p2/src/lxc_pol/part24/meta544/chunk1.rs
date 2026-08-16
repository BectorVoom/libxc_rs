//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1607/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1607(t76965: f64, t6071: f64, t18800: f64, t23384: f64, t23414: f64, t2770: f64, t39549: f64, t4474: f64, t50155: f64, t50166: f64, t50178: f64, t6049: f64, t61324: f64, t61330: f64, t61337: f64, t61355: f64, t75950: f64, t75956: f64, t75961: f64, t865: f64) -> (f64, f64) {
    let t87318 = 96.0_f64 * t76965;
    let t87337 = t6071 * t6071;
    let t87342 = 0.21951497276451705328e-1_f64 * t75950 - 0.44178176337912614788e-3_f64 * t50155 - 0.39029762157531132075e-2_f64 * t61324 - 0.26341796731742046395e1_f64 * t4474 * t23384 - 0.78059524315062264152e-1_f64 * t61330 - 0.68293547082294194357e-1_f64 * t50166 - 0.11708928647259339623e0_f64 * t75956 + 0.79025390195226139183e1_f64 * t18800 * t6049 - 0.78548797528808629095e-3_f64 * t50178 - 0.87805989105806821314e-1_f64 * t61337 - 0.15805078039045227836e2_f64 * t4474 * t23414 + 0.39029762157531132076e-1_f64 * t75961 - t39549 + 0.39512695097613069591e1_f64 * t865 * t2770 * t87337 + 0.78059524315062264152e-1_f64 * t61355;
    (t87318, t87342)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 491/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk491(t236: f64, t2627: f64, t232: f64, t815: f64, t835: f64, t812: f64, t831: f64, t242: f64, t67: f64, t845: f64, t246: f64, t120: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2628 = t2627 * t236;
    let t2632 = t232 * t232;
    let t2638 = t815 * t835;
    let t2639 = t812 * t2638;
    let t2640 = t2639 * t831;
    let t2642 = t815 * t242;
    let t2643 = t812 * t2642;
    let t2644 = t845 * t67;
    let t2645 = t2644 * t246;
    let t2646 = t120 * t828;
    (t2628, t2632, t2639, t2640, t2643, t2645, t2646)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 626/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk626(t208: f64, t2739: f64, t218: f64, t219: f64, t1833: f64, t1870: f64, t1881: f64, t1883: f64, t2730: f64, t2741: f64, t2755: f64, t2760: f64, t2766: f64, t2768: f64, t2772: f64, t2776: f64) -> (f64, f64, f64) {
    let t2778 = t208 * t2739;
    let t2780 = t218 * t219 * t2778;
    let t2782 = -0.9494625e0_f64 * t2755 + 0.1898925e1_f64 * t2760 + t1870 - 0.29896666666666666667e0_f64 * t1833 - 0.29896666666666666667e0_f64 * t2730 + 0.8969e0_f64 * t2741 + 0.15358125e0_f64 * t2766 + 0.3071625e0_f64 * t2768 + t1881 - 0.16431333333333333333e0_f64 * t1883 - 0.16431333333333333333e0_f64 * t2772 + 0.24647e0_f64 * t2776 + 0.24647e0_f64 * t2780;
    (t2778, t2780, t2782)
}

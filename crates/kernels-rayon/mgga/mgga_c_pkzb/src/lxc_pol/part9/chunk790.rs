//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 790/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk790(t2070: f64, t771: f64, t2006: f64, t2074: f64, t279: f64, t2887: f64, t303: f64, t5591: f64, t5592: f64, t5595: f64, t5597: f64, t5604: f64, t5607: f64, t5609: f64, t5614: f64, t5617: f64) -> (f64, f64) {
    let t5620 = t771 * t2070;
    let t5626 = t5591 - 77.0_f64 / 162.0_f64 * t5592 * t279 + 11.0_f64 / 108.0_f64 * t5595 + t5597 / 54.0_f64 - 0.53100265402527852012e-1_f64 * t5604 * t303 + 0.14481890564325777821e-1_f64 * t5607 + 0.7622047665434619906e-3_f64 * t5609 + t5614 + t2887 * t5617 / 16.0_f64 + 0.91464571985215438873e-2_f64 * t5620 + 0.68598428988911579154e-2_f64 * t771 * t2074 - 0.20579528696673473747e-1_f64 * t771 * t2006;
    (t5620, t5626)
}

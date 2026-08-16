//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2328;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta602(t39597: f64, t786: f64, t10665: f64, t675: f64, t10871: f64, t268: f64, t10530: f64, t2723: f64, t4503: f64, t860: f64, t10532: f64, t10542: f64, t10547: f64, t10638: f64, t231: f64, t2798: f64, t2645: f64, t837: f64, t2782: f64, t2797: f64, t10115: f64, t883: f64, t2482: f64, t2811: f64, t39588: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39599, t39602, t39606, t39608, t39610, t39612) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2328(t39597, t786, t10665, t675, t10871, t268, t10530, t2723, t4503, t860, t10532, t10542, t10547);
        let (t39617, t39620, t39622, t39624, t39629) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2329(t10638, t231, t268, t2798, t675, t2645, t837, t2782, t2797, t10115, t883, t2482, t2811, t39588, t686, t72);
    (t39599, t39602, t39606, t39608, t39610, t39612, t39617, t39620, t39622, t39624, t39629)
}

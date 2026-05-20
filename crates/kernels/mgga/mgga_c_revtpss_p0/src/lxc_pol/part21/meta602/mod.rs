//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2328;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta602<F: Float>(t39597: F, t786: F, t10665: F, t675: F, t10871: F, t268: F, t10530: F, t2723: F, t4503: F, t860: F, t10532: F, t10542: F, t10547: F, t10638: F, t231: F, t2798: F, t2645: F, t837: F, t2782: F, t2797: F, t10115: F, t883: F, t2482: F, t2811: F, t39588: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39599, t39602, t39606, t39608, t39610, t39612) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2328::<F>(t39597, t786, t10665, t675, t10871, t268, t10530, t2723, t4503, t860, t10532, t10542, t10547);
        let (t39617, t39620, t39622, t39624, t39629) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2329::<F>(t10638, t231, t268, t2798, t675, t2645, t837, t2782, t2797, t10115, t883, t2482, t2811, t39588, t686, t72);
    (t39599, t39602, t39606, t39608, t39610, t39612, t39617, t39620, t39622, t39624, t39629)
}

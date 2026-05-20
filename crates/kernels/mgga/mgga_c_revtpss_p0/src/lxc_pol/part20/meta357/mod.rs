//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1300;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta357<F: Float>(t231: F, t268: F, t2798: F, t793: F, t836: F, t215: F, t2722: F, t2645: F, t4366: F, t10529: F, t2782: F, t14545: F, t251: F, t786: F, t10665: F, t675: F, t10871: F, t10530: F, t2723: F, t4503: F, t860: F, t10532: F, t10542: F, t10547: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39581, t39583, t39586, t39588, t39590, t39595, t39597) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1300::<F>(t231, t268, t2798, t793, t836, t215, t2722, t2645, t4366, t10529, t2782, t14545, t251);
        let (t39599, t39602, t39606, t39610, t39612) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1301::<F>(t39597, t786, t10665, t675, t10871, t268, t10530, t2723, t4503, t860, t10532, t10542, t10547);
    (t39581, t39583, t39586, t39588, t39590, t39595, t39599, t39602, t39606, t39610, t39612)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta876 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3041;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta876<F: Float>(t10542: F, t14563: F, t14519: F, t2470: F, t2798: F, t231: F, t51049: F, t2782: F, t2797: F, t14663: F, t686: F, t72: F, t4522: F, t874: F, t9288: F, t1573: F, t40317: F, t14587: F, t39608: F, t10069: F, t14496: F, t14524: F, t39575: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51429, t51434, t51436, t51438, t51442) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3041::<F>(t10542, t14563, t14519, t2470, t2798, t231, t51049, t2782, t2797, t14663, t686, t72);
        let (t51445, t51452, t51460, t51470, t51483) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042::<F>(t4522, t874, t9288, t1573, t40317, t14587, t2782, t39608, t10069, t14496, t14524, t39575);
    (t51429, t51434, t51436, t51438, t51442, t51445, t51452, t51460, t51470, t51483)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1033 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1033<F: Float>(t20343: F, t698: F, t20346: F, t141: F, t3417: F, t68355: F, t12254: F, t68340: F, t1134: F, t5079: F, t16851: F, t16854: F, t58207: F, t68454: F, t68529: F, t68532: F, t68535: F, t68538: F, t68540: F, t68543: F, t68546: F) -> (F, F, F, F, F, F, F) {
        let (t68548, t68550, t68553, t68556, t68559, t68561) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616::<F>(t20343, t698, t20346, t141, t3417, t68355, t12254, t68340, t1134, t5079, t16851, t16854);
        let t68564 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617::<F>(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
    (t68548, t68550, t68553, t68556, t68559, t68561, t68564)
}

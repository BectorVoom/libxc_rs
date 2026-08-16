//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta254<F: Float>(t240: F, t624: F, t281: F, t283: F, t2909: F, t698: F, t3252: F, t11145: F, t141: F, t11169: F, t930: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11334: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11335, t11337, t11339, t11341, t11342, t11343, t11345, t11346, t11349) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1087::<F>(t240, t624, t281, t283, t2909, t698, t3252, t11145, t141, t11169, t930, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11334);
    (t11335, t11337, t11339, t11341, t11342, t11343, t11345, t11346, t11349)
}

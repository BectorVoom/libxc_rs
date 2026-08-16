//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta336<F: Float>(t240: F, t3252: F, t11145: F, t141: F, t11169: F, t930: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11334: F, t11338: F, t11339: F, t276: F, t285: F, t2881: F, t918: F, t273: F, t2439: F, t931: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11341, t11342, t11343, t11345, t11346, t11349) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1648::<F>(t240, t3252, t11145, t141, t11169, t930, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11334, t11338, t11339);
        let (t11354, t11355, t11356, t11358, t11359, t11366) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1649::<F>(t276, t285, t2881, t918, t273, t2439, t931);
    (t11341, t11342, t11343, t11345, t11346, t11349, t11354, t11355, t11356, t11358, t11359, t11366)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta562<F: Float>(t28329: F, t6637: F, t6552: F, t1894: F, t5631: F, t214: F, t1880: F, t1510: F, t25249: F, t6646: F, t22986: F, t5527: F, t6638: F, t23035: F, t1484: F, t25319: F, t25255: F, t1499: F, t23014: F, t23032: F, t25246: F, t25259: F, t28323: F, t4166: F, t7533: F, t7535: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28330, t28331, t28333, t28334, t28335, t28337, t28338, t28339, t28341) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1923::<F>(t28329, t6637, t6552, t1894, t5631, t214, t1880, t1510, t25249, t6646, t22986, t5527, t6638);
        let (t28342, t28345, t28346, t28351, t28354) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1924::<F>(t28341, t6637, t23035, t1484, t25319, t6552, t1510, t25255, t1499, t23014, t23032, t25246, t25259, t28323, t28331, t28335, t28339, t4166, t7533, t7535, t812);
    (t28330, t28333, t28334, t28337, t28338, t28341, t28342, t28345, t28346, t28351, t28354)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1923;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta562(t28329: f64, t6637: f64, t6552: f64, t1894: f64, t5631: f64, t214: f64, t1880: f64, t1510: f64, t25249: f64, t6646: f64, t22986: f64, t5527: f64, t6638: f64, t23035: f64, t1484: f64, t25319: f64, t25255: f64, t1499: f64, t23014: f64, t23032: f64, t25246: f64, t25259: f64, t28323: f64, t4166: f64, t7533: f64, t7535: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28330, t28331, t28333, t28334, t28335, t28337, t28338, t28339, t28341) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1923(t28329, t6637, t6552, t1894, t5631, t214, t1880, t1510, t25249, t6646, t22986, t5527, t6638);
        let (t28342, t28345, t28346, t28351, t28354) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1924(t28341, t6637, t23035, t1484, t25319, t6552, t1510, t25255, t1499, t23014, t23032, t25246, t25259, t28323, t28331, t28335, t28339, t4166, t7533, t7535, t812);
    (t28330, t28333, t28334, t28337, t28338, t28341, t28342, t28345, t28346, t28351, t28354)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta827(t6244: f64, t905: f64, t11774: f64, t4782: f64, t53391: f64, t1011: f64, t15993: f64, t18909: f64, t11933: f64, t19976: f64, t3115: f64, t42793: f64, t6272: f64, t11922: f64, t16081: f64, t19749: f64, t20020: f64, t3211: f64, t15656: f64, t4845: f64, t19675: f64, t372: f64, t11947: f64, t20016: f64, t11875: f64, t19757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66966, t66972, t66981, t67006, t67015) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683(t6244, t905, t11774, t4782, t53391, t1011, t15993, t18909, t11933, t19976, t3115, t42793, t6272);
        let (t67025, t67044, t67048, t67052, t67072, t67152) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684(t11922, t16081, t19749, t20020, t3211, t15656, t4845, t19675, t372, t11947, t20016, t11875, t19757);
    (t66966, t66972, t66981, t67006, t67015, t67025, t67044, t67048, t67052, t67072, t67152)
}

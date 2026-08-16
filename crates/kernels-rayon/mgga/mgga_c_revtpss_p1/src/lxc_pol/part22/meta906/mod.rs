//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta906 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3105;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta906(t1045: f64, t606: f64, t11937: f64, t15671: f64, t11262: f64, t3127: f64, t4824: f64, t1065: f64, t15648: f64, t15772: f64, t3188: f64, t1063: f64, t16195: f64, t3172: f64, t16200: f64, t15775: f64, t16204: f64, t16209: f64, t11922: f64, t11927: f64, t15621: f64, t11671: f64, t4954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54397, t54407, t54414, t54419, t54432, t54435) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3105(t1045, t606, t11937, t15671, t11262, t3127, t4824, t1065, t15648, t15772, t3188, t1063, t16195, t3172);
        let (t54438, t54440, t54443, t54446, t54469, t54471) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3106(t1063, t16200, t3172, t15775, t3188, t16204, t16209, t11922, t11927, t15621, t11671, t4954);
    (t54397, t54407, t54414, t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54469, t54471)
}

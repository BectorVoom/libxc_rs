//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2097;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta648(t17435: f64, t7613: f64, t17303: f64, t29062: f64, t3678: f64, t17209: f64, t26880: f64, t29019: f64, t3707: f64, t26873: f64, t5265: f64, t15687: f64, t26865: f64, t3767: f64, t3782: f64, t1224: f64, t139: f64, t29047: f64, t5052: f64, t3698: f64, t5047: f64, t26866: f64, t5436: f64, t17225: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104817, t104825, t104828, t104833, t104834, t104844, t104852) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2097(t17435, t7613, t17303, t29062, t3678, t17209, t26880, t29019, t3707, t26873, t5265, t15687, t26865);
        let (t104853, t104856, t104863, t104872, t104888, t104894) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2098(t104852, t3767, t3782, t1224, t139, t29047, t5052, t3698, t5047, t26866, t5436, t17225, t7624);
    (t104817, t104825, t104828, t104833, t104834, t104844, t104853, t104856, t104863, t104872, t104888, t104894)
}

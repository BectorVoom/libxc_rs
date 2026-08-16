//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1151;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta345(t15191: f64, t1058: f64, t4794: f64, t11243: f64, t72: f64, t3088: f64, t12078: f64, t1086: f64, t4746: f64, t3090: f64, t1065: f64, t2852: f64, t3173: f64, t4879: f64, t4866: f64, t73: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15876, t15892, t15904, t15905, t15906, t15926, t15935) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1151(t15191, t1058, t4794, t11243, t72, t3088, t12078, t1086, t4746, t3090, t1065, t2852);
        let (t15942, t15957, t15986, t15990, t15993) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1152(t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140, t4579, t1011, t3252);
    (t15876, t15892, t15904, t15905, t15906, t15926, t15935, t15942, t15957, t15986, t15990, t15993)
}

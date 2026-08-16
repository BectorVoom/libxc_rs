//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta819(t11277: f64, t19826: f64, t16163: f64, t4879: f64, t1063: f64, t19681: f64, t3172: f64, t11710: f64, t19625: f64, t4899: f64, t19687: f64, t15772: f64, t4834: f64, t1065: f64, t19380: f64, t1062: f64, t19463: f64, t19730: f64, t3091: f64, t20050: f64, t3188: f64, t20054: f64, t18946: f64, t247: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65618, t65627, t65630, t65637, t65650, t65689) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667(t11277, t19826, t16163, t4879, t1063, t19681, t3172, t11710, t19625, t4899, t19687, t15772, t4834);
        let (t65712, t65717, t65738, t65801, t65803, t65807) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668(t1065, t19380, t1062, t19463, t11710, t19730, t3091, t20050, t3188, t20054, t1063, t18946, t247, t3109);
    (t65618, t65627, t65630, t65637, t65650, t65689, t65712, t65717, t65738, t65801, t65803, t65807)
}

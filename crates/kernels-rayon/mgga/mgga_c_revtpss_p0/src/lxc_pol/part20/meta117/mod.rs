//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk676;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta117(t3230: f64, t351: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t2251: f64, t1012: f64, t1010: f64, t614: f64, t1016: f64, t140: f64, t1011: f64, t1015: f64, t2258: f64, t271: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3231, t3234, t3236, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk676(t3230, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
        let (t3244, t3245, t3247, t3248, t3252) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk677(t1016, t140, t1011, t1015, t2258, t1012, t271, t905);
    (t3231, t3234, t3236, t3237, t3238, t3241, t3244, t3245, t3247, t3248, t3252)
}

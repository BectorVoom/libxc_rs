//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2677;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta824(t11773: f64, t4954: f64, t1011: f64, t6284: f64, t697: f64, t19900: f64, t3241: f64, t19477: f64, t3153: f64, t15905: f64, t56017: f64, t55899: f64, t15700: f64, t19992: f64, t53405: f64, t16226: f64, t19997: f64, t11710: f64, t19777: f64, t3091: f64, t19644: f64, t140: f64, t19916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66542, t66547, t66551, t66565, t66621, t66624) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2677(t11773, t4954, t1011, t6284, t697, t19900, t3241, t19477, t3153, t15905, t56017, t55899);
        let (t66644, t66647, t66655, t66660, t66686) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2678(t15700, t19992, t53405, t16226, t19997, t11710, t19777, t3091, t19644, t1011, t140, t19916);
    (t66542, t66547, t66551, t66565, t66621, t66624, t66644, t66647, t66655, t66660, t66686)
}

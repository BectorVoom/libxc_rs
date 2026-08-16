//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta838 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2710;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta838(t17373: f64, t21203: f64, t1230: f64, t21271: f64, t1263: f64, t21082: f64, t17544: f64, t5293: f64, t21275: f64, t17769: f64, t5381: f64, t5391: f64, t1247: f64, t20902: f64, t3172: f64, t1234: f64, t17209: f64, t17505: f64, t12855: f64, t12916: f64, t21120: f64, t21093: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69721, t69723, t69742, t69773, t69783, t69787, t69789) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2710(t17373, t21203, t1230, t21271, t1263, t21082, t17544, t5293, t21275, t17769, t5381, t5391);
        let (t69793, t69795, t69812, t69820, t69832) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2711(t1247, t20902, t3172, t1234, t21271, t17209, t17505, t12855, t12916, t21120, t21093, t372);
    (t69721, t69723, t69742, t69773, t69783, t69787, t69789, t69793, t69795, t69812, t69820, t69832)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1018;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta252(t11986: f64, t1592: f64, t247: f64, t1063: f64, t1062: f64, t11940: f64, t11262: f64, t1670: f64, t1041: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t1647: f64, t3140: f64, t3149: f64, t1660: f64, t3201: f64, t11243: f64, t72: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15711, t15712, t15716, t15731, t15732, t15749) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1018(t11986, t1592, t247, t1063, t1062, t11940, t11262, t1670, t1041, t1663, t371, t676);
        let (t15750, t15822, t15823, t15862, t15904, t15905) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1019(t1025, t15749, t1647, t3140, t3149, t1660, t3201, t11243, t72, t3088);
    (t15711, t15712, t15716, t15731, t15732, t15749, t15750, t15822, t15823, t15862, t15904, t15905)
}

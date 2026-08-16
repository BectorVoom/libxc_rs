//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta759 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta759(t43347: f64, t53668: f64, t11852: f64, t41270: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t3090: f64, t11262: f64, t3127: f64, t4874: f64, t15749: f64, t3211: f64, t16199: f64, t372: f64, t16208: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64, t225: f64, t53166: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54509, t54537, t54570, t54578, t54599) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552(t43347, t53668, t11852, t41270, t3316, t4746, t4891, t16381, t3090, t11262, t3127, t4874);
        let (t54648, t54658, t54672, t54687, t54695, t54696) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553(t15749, t3211, t16199, t372, t16208, t1025, t1663, t2434, t371, t225, t53166, t366);
    (t54509, t54537, t54570, t54578, t54599, t54648, t54658, t54672, t54687, t54695, t54696)
}

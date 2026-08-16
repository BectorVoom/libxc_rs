//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta370(t3211: f64, t4845: f64, t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t12116: f64, t4891: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15744, t15745, t15749, t15750, t15752, t15754, t15758) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1699(t3211, t4845, t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t12116, t4891);
    (t15744, t15745, t15749, t15750, t15752, t15754, t15758)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta472(t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t1028: f64, t11779: f64, t11792: f64, t11994: f64, t15724: f64, t15725: f64, t15728: f64, t15732: f64, t15736: f64, t15744: f64, t15745: f64, t1665: f64, t4839: f64, t4875: f64, t12116: f64, t4891: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15749, t15750, t15752, t15754, t15755) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2171(t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t15744, t15745, t1665, t4839, t4875);
        let t15758 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2172(t12116, t4891);
    (t15749, t15750, t15752, t15754, t15755, t15758)
}

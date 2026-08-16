//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2103;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2104;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2105;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta498(t3105: f64, t3204: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t1028: f64, t11779: f64, t11792: f64, t11994: f64, t15724: f64, t15725: f64, t1665: f64, t4839: f64, t4875: f64, t12116: f64, t4891: f64, t4874: f64, t4802: f64, t1063: f64, t4807: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t11737: f64, t11745: f64, t3106: f64, t4803: f64, t4808: f64, t4896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15728, t15731, t15732, t15734, t15736, t15744, t15745) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2103(t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845, t1053, t4857);
        let (t15749, t15752, t15755) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2104(t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t15744, t15745, t1665, t4839, t4875);
        let t15758 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2105(t12116, t4891);
        let (t15769, t15772, t15775, t15779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2106(t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t15758, t3106, t4803, t4808, t4896);
    (t15728, t15731, t15734, t15745, t15749, t15752, t15755, t15758, t15769, t15772, t15775, t15779)
}

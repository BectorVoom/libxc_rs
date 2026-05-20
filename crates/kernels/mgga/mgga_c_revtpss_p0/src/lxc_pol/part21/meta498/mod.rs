//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2103;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2104;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2105;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta498<F: Float>(t3105: F, t3204: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F, t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t1028: F, t11779: F, t11792: F, t11994: F, t15724: F, t15725: F, t1665: F, t4839: F, t4875: F, t12116: F, t4891: F, t4874: F, t4802: F, t1063: F, t4807: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11745: F, t3106: F, t4803: F, t4808: F, t4896: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15728, t15731, t15732, t15734, t15736, t15744, t15745) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2103::<F>(t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845, t1053, t4857);
        let (t15749, t15752, t15755) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2104::<F>(t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t15744, t15745, t1665, t4839, t4875);
        let t15758 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2105::<F>(t12116, t4891);
        let (t15769, t15772, t15775, t15779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2106::<F>(t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t15758, t3106, t4803, t4808, t4896);
    (t15728, t15731, t15734, t15745, t15749, t15752, t15755, t15758, t15769, t15772, t15775, t15779)
}

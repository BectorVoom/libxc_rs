//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta472<F: Float>(t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t1028: F, t11779: F, t11792: F, t11994: F, t15724: F, t15725: F, t15728: F, t15732: F, t15736: F, t15744: F, t15745: F, t1665: F, t4839: F, t4875: F, t12116: F, t4891: F) -> (F, F, F, F, F, F) {
        let (t15749, t15750, t15752, t15754, t15755) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2171::<F>(t1663, t371, t676, t1025, t11922, t4901, t4899, t1028, t11779, t11792, t11994, t15724, t15725, t15728, t15732, t15736, t15744, t15745, t1665, t4839, t4875);
        let t15758 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2172::<F>(t12116, t4891);
    (t15749, t15750, t15752, t15754, t15755, t15758)
}

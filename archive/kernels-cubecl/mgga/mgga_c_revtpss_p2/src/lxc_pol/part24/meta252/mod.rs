//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1018;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta252<F: Float>(t11986: F, t1592: F, t247: F, t1063: F, t1062: F, t11940: F, t11262: F, t1670: F, t1041: F, t1663: F, t371: F, t676: F, t1025: F, t1647: F, t3140: F, t3149: F, t1660: F, t3201: F, t11243: F, t72: F, t3088: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15711, t15712, t15716, t15731, t15732, t15749) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1018::<F>(t11986, t1592, t247, t1063, t1062, t11940, t11262, t1670, t1041, t1663, t371, t676);
        let (t15750, t15822, t15823, t15862, t15904, t15905) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1019::<F>(t1025, t15749, t1647, t3140, t3149, t1660, t3201, t11243, t72, t3088);
    (t15711, t15712, t15716, t15731, t15732, t15749, t15750, t15822, t15823, t15862, t15904, t15905)
}

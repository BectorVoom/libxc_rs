//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1696;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1697;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta369<F: Float>(t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11940: F, t3111: F, t4834: F, t11788: F, t3105: F, t3204: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t15707 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1696::<F>(t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1697::<F>(t11986, t1592, t247, t1063, t1062, t11940);
        let (t15724, t15725, t15728, t15731, t15732, t15734, t15736) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1698::<F>(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
    (t15707, t15711, t15712, t15716, t15724, t15725, t15728, t15731, t15732, t15734, t15736)
}

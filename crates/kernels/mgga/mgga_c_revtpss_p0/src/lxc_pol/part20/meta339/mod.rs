//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1264;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta339<F: Float>(t15687: F, t3088: F, t3317: F, t12131: F, t3095: F, t1087: F, t11773: F, t372: F, t4801: F, t1062: F, t11940: F, t11788: F, t3105: F, t3204: F, t12116: F, t4891: F, t3133: F, t3154: F, t11243: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15688, t15689, t15692, t15700, t15701, t15716, t15725) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1264::<F>(t15687, t3088, t3317, t12131, t3095, t1087, t11773, t372, t4801, t1062, t11940, t11788);
        let (t15728, t15758, t15785, t15904, t15905) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1265::<F>(t3105, t3204, t12116, t4891, t3133, t3154, t11243, t72, t3088);
    (t15688, t15689, t15692, t15700, t15701, t15716, t15725, t15728, t15758, t15785, t15904, t15905)
}

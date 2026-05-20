//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta514<F: Float>(t1065: F, t23598: F, t11630: F, t23829: F, t3172: F, t1011: F, t140: F, t24016: F, t11710: F, t23907: F, t3091: F, t23912: F, t1668: F, t905: F, t11774: F, t53391: F, t6267: F, t19968: F, t4817: F, t20054: F, t4834: F, t19882: F, t1062: F, t23960: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79301, t79309, t79315, t79428, t79439) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532::<F>(t1065, t23598, t11630, t23829, t3172, t1011, t140, t24016, t11710, t23907, t3091, t23912);
        let (t79450, t79474, t79546, t79548, t79553, t79559) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533::<F>(t1668, t905, t11774, t53391, t6267, t19968, t4817, t20054, t4834, t19882, t1062, t23960);
    (t79301, t79309, t79315, t79428, t79439, t79450, t79474, t79546, t79548, t79553, t79559)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta473<F: Float>(t1045: F, t4579: F, t15691: F, t1043: F, t1592: F, t3155: F, t4817: F, t4834: F, t11933: F, t11956: F, t11967: F, t11972: F, t11989: F, t15700: F, t15830: F, t16121: F, t16226: F, t1675: F, t3211: F, t6273: F, t6278: F) -> (F, F, F, F, F, F, F) {
        let (t19992, t19993, t19996, t19997, t19998, t20005, t20012) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1924::<F>(t1045, t4579, t15691, t1043, t1592, t3155, t4817, t4834, t11933, t11956, t11967, t11972, t11989, t15700, t15830, t16121, t16226, t1675, t3211, t6273, t6278);
    (t19992, t19993, t19996, t19997, t19998, t20005, t20012)
}

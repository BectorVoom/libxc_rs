//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta518<F: Float>(t16237: F, t225: F, t385: F, t1096: F, t4772: F, t1079: F, t1651: F, t3269: F, t3270: F, t5015: F, t1073: F, t1076: F, t11190: F, t11224: F, t15579: F, t15886: F, t1647: F, t1652: F, t3047: F, t3052: F, t3063: F, t3261: F, t342: F, t386: F, t4743: F, t4758: F, t4764: F, t4932: F, t4941: F, t4947: F, t989: F, t995: F) -> (F, F, F, F, F) {
        let (t16239, t16243, t16249, t16255, t16272) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2154::<F>(t16237, t225, t385, t1096, t4772, t1079, t1651, t3269, t3270, t5015, t1073, t1076, t11190, t11224, t15579, t15886, t1647, t1652, t3047, t3052, t3063, t3261, t342, t386, t4743, t4758, t4764, t4932, t4941, t4947, t989, t995);
    (t16239, t16243, t16249, t16255, t16272)
}

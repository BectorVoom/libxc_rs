//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2555;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2556;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta633<F: Float>(t19508: F, t19554: F, t19606: F, t20149: F, t1079: F, t20112: F, t225: F, t385: F, t1096: F, t6392: F, t3269: F, t1647: F, t1678: F, t378: F, t6235: F, t1076: F, t1097: F, t11187: F, t16340: F, t16374: F, t1652: F, t16597: F, t1696: F, t19856: F, t3264: F, t342: F, t386: F, t4778: F, t4932: F, t4941: F, t6245: F, t6345: F, t6351: F, t989: F) -> (F, F, F, F, F, F, F) {
        let (t20151, t20152, t20168, t20172, t20175) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2555::<F>(t19508, t19554, t19606, t20149, t1079, t20112, t225, t385, t1096, t6392, t3269, t1647, t1678);
        let (t20178, t20187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2556::<F>(t378, t6235, t1076, t1097, t11187, t16340, t16374, t1647, t1652, t16597, t1696, t19856, t20152, t20168, t20172, t20175, t3264, t342, t386, t4778, t4932, t4941, t6245, t6345, t6351, t989);
    (t20151, t20152, t20168, t20172, t20175, t20178, t20187)
}

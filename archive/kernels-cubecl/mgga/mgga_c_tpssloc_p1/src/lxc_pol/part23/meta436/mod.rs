//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta436<F: Float>(t11697: F, t22153: F, t3577: F, t13969: F, t22274: F, t3515: F, t1227: F, t22196: F, t1222: F, t22015: F, t20246: F, t972: F, t1193: F, t22104: F, t22038: F, t3448: F, t20234: F, t44607: F, t15376: F, t18446: F, t15338: F, t18427: F, t3447: F, t22032: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73084, t73096, t73099, t73102, t73113) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277::<F>(t11697, t22153, t3577, t13969, t22274, t3515, t1227, t22196, t1222, t22015, t20246, t972);
        let (t73142, t73169, t73181, t73188, t73199, t73201) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1278::<F>(t1193, t22104, t22038, t3448, t20234, t44607, t15376, t18446, t15338, t18427, t3447, t22032);
    (t73084, t73096, t73099, t73102, t73113, t73142, t73169, t73181, t73188, t73199, t73201)
}

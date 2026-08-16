//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2243;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta680<F: Float>(t14080: F, t4571: F, t14202: F, t4644: F, t10413: F, t10422: F, t17700: F, t1036: F, t17878: F, t13969: F, t17631: F, t3039: F, t3082: F, t5905: F, t10403: F, t18035: F, t17906: F, t3048: F, t1041: F, t248: F, t43338: F, t5677: F, t3070: F, t43198: F, t5908: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62282, t62284, t62306, t62343, t62349) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2243::<F>(t14080, t4571, t14202, t4644, t10413, t10422, t17700, t1036, t17878, t13969, t17631, t3039);
        let (t62360, t62418, t62441, t62445, t62494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244::<F>(t3082, t5905, t10403, t10422, t18035, t17906, t3048, t1041, t248, t43338, t5677, t3070, t43198, t5908);
    (t62282, t62284, t62306, t62343, t62349, t62360, t62418, t62441, t62445, t62494)
}

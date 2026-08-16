//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2037;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta465<F: Float>(t16080: F, t16121: F, t225: F, t3856: F, t5335: F, t3851: F, t5348: F, t1332: F, t1336: F, t1381: F, t16033: F, t16037: F, t16041: F, t16044: F, t16047: F, t16049: F, t16052: F, t16055: F, t16060: F, t16065: F, t16068: F, t3777: F, t3902: F, t5234: F, t5334: F, t5336: F, t5344: F, t5345: F, t5349: F, t5351: F, t564: F, t1338: F, t5318: F, t1352: F, t12259: F, t1825: F, t3866: F, t5310: F, t1307: F, t5187: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16122, t16123, t16125, t16127, t16131) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2037::<F>(t16080, t16121, t225, t3856, t5335, t3851, t5348, t1332, t1336, t1381, t16033, t16037, t16041, t16044, t16047, t16049, t16052, t16055, t16060, t16065, t16068, t3777, t3902, t5234, t5334, t5336, t5344, t5345, t5349, t5351, t564);
        let (t16132, t16133, t16136, t16147, t16148) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2038::<F>(t1338, t5318, t1352, t12259, t1825, t3866, t5310, t1307, t5187);
    (t16122, t16123, t16125, t16127, t16131, t16132, t16133, t16136, t16147, t16148)
}

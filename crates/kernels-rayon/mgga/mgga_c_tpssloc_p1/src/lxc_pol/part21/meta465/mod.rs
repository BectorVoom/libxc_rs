//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2037;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta465(t16080: f64, t16121: f64, t225: f64, t3856: f64, t5335: f64, t3851: f64, t5348: f64, t1332: f64, t1336: f64, t1381: f64, t16033: f64, t16037: f64, t16041: f64, t16044: f64, t16047: f64, t16049: f64, t16052: f64, t16055: f64, t16060: f64, t16065: f64, t16068: f64, t3777: f64, t3902: f64, t5234: f64, t5334: f64, t5336: f64, t5344: f64, t5345: f64, t5349: f64, t5351: f64, t564: f64, t1338: f64, t5318: f64, t1352: f64, t12259: f64, t1825: f64, t3866: f64, t5310: f64, t1307: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16122, t16123, t16125, t16127, t16131) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2037(t16080, t16121, t225, t3856, t5335, t3851, t5348, t1332, t1336, t1381, t16033, t16037, t16041, t16044, t16047, t16049, t16052, t16055, t16060, t16065, t16068, t3777, t3902, t5234, t5334, t5336, t5344, t5345, t5349, t5351, t564);
        let (t16132, t16133, t16136, t16147, t16148) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2038(t1338, t5318, t1352, t12259, t1825, t3866, t5310, t1307, t5187);
    (t16122, t16123, t16125, t16127, t16131, t16132, t16133, t16136, t16147, t16148)
}

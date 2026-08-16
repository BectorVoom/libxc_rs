//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta765(t16391: f64, t16398: f64, t12283: f64, t16244: f64, t3862: f64, t5231: f64, t16356: f64, t3726: f64, t12328: f64, t1815: f64, t16397: f64, t3777: f64, t5252: f64, t1336: f64, t2691: f64, t3788: f64, t16028: f64, t225: f64, t40041: f64, t544: f64, t68: f64, t1332: f64, t16046: f64, t1338: f64, t16413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54750, t54764, t54785, t54787, t54793, t54801) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642(t16391, t16398, t12283, t16244, t3862, t5231, t16356, t3726, t12328, t1815, t16397, t3777, t5252);
        let (t54811, t54825, t54963, t54976, t55039) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643(t1336, t2691, t3788, t5252, t16028, t225, t40041, t544, t68, t1332, t16046, t1338, t16413);
    (t54750, t54764, t54785, t54787, t54793, t54801, t54811, t54825, t54963, t54976, t55039)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta635(t3862: f64, t5231: f64, t12328: f64, t1815: f64, t1336: f64, t2691: f64, t3788: f64, t5252: f64, t3787: f64, t5318: f64, t40041: f64, t544: f64, t68: f64, t12020: f64, t1842: f64, t1307: f64, t193: f64, t111: f64, t5363: f64, t6470: f64, t19530: f64, t626: f64, t1447: f64, t2349: f64, t2281: f64, t5489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54786, t54793, t54812, t54905, t54963) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172(t3862, t5231, t12328, t1815, t1336, t2691, t3788, t5252, t3787, t5318, t40041, t544, t68);
        let (t55118, t55224, t55353, t55388, t55420, t55491, t55531) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2173(t12020, t1842, t1307, t193, t111, t5363, t6470, t19530, t626, t1447, t2349, t2281, t5489);
    (t54786, t54793, t54812, t54905, t54963, t55118, t55224, t55353, t55388, t55420, t55491, t55531)
}

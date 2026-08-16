//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2243;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta680(t14080: f64, t4571: f64, t14202: f64, t4644: f64, t10413: f64, t10422: f64, t17700: f64, t1036: f64, t17878: f64, t13969: f64, t17631: f64, t3039: f64, t3082: f64, t5905: f64, t10403: f64, t18035: f64, t17906: f64, t3048: f64, t1041: f64, t248: f64, t43338: f64, t5677: f64, t3070: f64, t43198: f64, t5908: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62282, t62284, t62306, t62343, t62349) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2243(t14080, t4571, t14202, t4644, t10413, t10422, t17700, t1036, t17878, t13969, t17631, t3039);
        let (t62360, t62418, t62441, t62445, t62494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244(t3082, t5905, t10403, t10422, t18035, t17906, t3048, t1041, t248, t43338, t5677, t3070, t43198, t5908);
    (t62282, t62284, t62306, t62343, t62349, t62360, t62418, t62441, t62445, t62494)
}

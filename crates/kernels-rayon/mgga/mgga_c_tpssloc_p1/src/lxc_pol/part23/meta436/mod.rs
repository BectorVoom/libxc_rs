//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta436(t11697: f64, t22153: f64, t3577: f64, t13969: f64, t22274: f64, t3515: f64, t1227: f64, t22196: f64, t1222: f64, t22015: f64, t20246: f64, t972: f64, t1193: f64, t22104: f64, t22038: f64, t3448: f64, t20234: f64, t44607: f64, t15376: f64, t18446: f64, t15338: f64, t18427: f64, t3447: f64, t22032: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73084, t73096, t73099, t73102, t73113) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277(t11697, t22153, t3577, t13969, t22274, t3515, t1227, t22196, t1222, t22015, t20246, t972);
        let (t73142, t73169, t73181, t73188, t73199, t73201) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1278(t1193, t22104, t22038, t3448, t20234, t44607, t15376, t18446, t15338, t18427, t3447, t22032);
    (t73084, t73096, t73099, t73102, t73113, t73142, t73169, t73181, t73188, t73199, t73201)
}

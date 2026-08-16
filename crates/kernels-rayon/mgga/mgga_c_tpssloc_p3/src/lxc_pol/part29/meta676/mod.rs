//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2266;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta676(t2314: f64, t26003: f64, t1874: f64, t90381: f64, t1983: f64, t2019: f64, t55169: f64, t510: f64, t652: f64, t86604: f64, t26114: f64, t6535: f64, t26179: f64, t25994: f64, t12823: f64, t7461: f64, t25980: f64, t4034: f64, t12813: f64, t89: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91724, t91726, t91730, t91735, t91737) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2266(t2314, t26003, t1874, t90381, t1983, t2019, t55169, t510, t652, t86604, t26114, t6535);
        let (t91739, t91747, t91749, t91752, t91755, t91757) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267(t26179, t6535, t2314, t25994, t12823, t7461, t25980, t4034, t12813, t89, t1874, t6525);
    (t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t91752, t91755, t91757)
}

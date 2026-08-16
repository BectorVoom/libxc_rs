//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk818;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta155(t4205: f64, t708: f64, t1462: f64, t2427: f64, t2373: f64, t2377: f64, t2408: f64, t4097: f64, t4099: f64, t4100: f64, t4103: f64, t4198: f64, t4201: f64, t4204: f64, t1474: f64, t67: f64, t758: f64, t2431: f64, t2532: f64, t2653: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4207, t4209, t4210) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk818(t4205, t708, t1462, t2427, t2373, t2377, t2408, t4097, t4099, t4100, t4103, t4198, t4201, t4204);
        let (t4211, t4212, t4213, t4214, t4215, t4216, t4217) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk819(t1474, t67, t758, t2431, t2532, t2653, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2538, t2665);
    (t4207, t4209, t4210, t4211, t4212, t4213, t4214, t4215, t4216, t4217)
}

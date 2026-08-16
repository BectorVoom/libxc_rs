//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk876;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta133(t40: f64, t52: f64, t4072: f64, t510: f64, t1774: f64, t671: f64, t1409: f64, t2433: f64, t3966: f64, t607: f64, t73: f64, t2440: f64, t76: f64, t157: f64, zeta_threshold: f64, t182: f64, t145: f64, t185: f64, t1472: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4073, t4077, t4080, t4087, t4094, t4095) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk876(t40, t52, t4072, t510, t1774, t671, t1409, t2433, t3966, t607, t73, t2440, t76, t157, zeta_threshold);
        let (t4097, t4098, t4099, t4100, t4101) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk877(t182, t4095, t145, t4094, t185, t1472, t751, t1409);
    (t4073, t4077, t4080, t4087, t4094, t4095, t4097, t4098, t4099, t4100, t4101)
}

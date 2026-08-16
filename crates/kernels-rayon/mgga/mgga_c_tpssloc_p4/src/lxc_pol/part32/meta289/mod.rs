//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta289(t116: f64, t131: f64, t9537: f64, t207: f64, t9534: f64, t2559: f64, t786: f64, t789: f64, t2566: f64, t2578: f64, t2570: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9538, t9540, t9541, t9542, t9546, t9547, t9549) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1301(t116, t131, t9537, t207, t9534, t2559, t786, t789, t2566, t2578, t2570, t792);
    (t9538, t9540, t9541, t9542, t9546, t9547, t9549)
}

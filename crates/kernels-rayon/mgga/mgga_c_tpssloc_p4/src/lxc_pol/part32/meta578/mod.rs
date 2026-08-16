//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1956;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta578(t29560: f64, t1932: f64, t2133: f64, t7573: f64, t8027: f64, t1737: f64, t2136: f64, t24681: f64, t24704: f64, t27578: f64, t27592: f64, t27599: f64, t27609: f64, t27614: f64, t6203: f64, t6211: f64, t7345: f64, t337: f64, t5415: f64, t131: f64, t475: f64, t6218: f64, t68: f64, t7328: f64, t1730: f64, t8048: f64, t2139: f64, t6163: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29561, t29562, t29563, t29569, t29580) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1956(t29560, t1932, t2133, t7573, t8027, t1737, t2136, t24681, t24704, t27578, t27592, t27599, t27609, t27614, t6203, t6211, t7345);
        let (t29584, t29585, t29593, t29594, t29597, t29600, t29601) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1957(t337, t5415, t131, t475, t6218, t68, t7328, t1730, t8048, t2139, t6163, t471);
    (t29561, t29562, t29563, t29569, t29580, t29584, t29585, t29593, t29594, t29597, t29600, t29601)
}

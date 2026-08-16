//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2074;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta604(t6787: f64, t82573: f64, t23384: f64, t23687: f64, t23658: f64, t23665: f64, t23494: f64, t6743: f64, t23547: f64, t23644: f64, t23647: f64, t1049: f64, t883: f64, t6790: f64, t221: f64, t697: f64, t1926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82574, t82576, t82590, t82592, t82596, t82605, t82618, t82625) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2074(t6787, t82573, t23384, t23687, t23658, t23665, t23494, t6743, t23547, t23644, t23647, t1049, t883);
        let (t82629, t82632) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2075(t6790, t82573, t221, t697, t1926);
    (t82574, t82576, t82590, t82592, t82596, t82605, t82618, t82625, t82629, t82632)
}

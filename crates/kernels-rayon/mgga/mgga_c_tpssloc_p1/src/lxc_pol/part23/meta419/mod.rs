//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1243;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta419(t21127: f64, t690: f64, t21131: f64, t699: f64, t21135: f64, t21139: f64, t21119: f64, t21697: f64, t3216: f64, t21238: f64, t2929: f64, t21334: f64, t892: f64, t21347: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68498 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1243(t21127, t690);
        let (t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244(t21131, t699, t21135, t21139, t21119, t21697, t3216, t21238, t2929, t21334, t892, t21347, t300);
    (t68498, t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012)
}

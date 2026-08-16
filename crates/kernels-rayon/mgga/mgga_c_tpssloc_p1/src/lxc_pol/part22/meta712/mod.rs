//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2311;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta712(t16606: f64, t17120: f64, t1877: f64, t40764: f64, t40766: f64, t4255: f64, t4303: f64, t4314: f64, t46292: f64, t67176: f64, t67178: f64, t67180: f64, t67183: f64, t67186: f64, t67191: f64, t152: f64, t20825: f64, t607: f64, t41284: f64, t46302: f64, t20742: f64, t67: f64, t758: f64, t58047: f64, t58052: f64, t58057: f64, t40794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t67195 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310(t16606, t17120, t1877, t40764, t40766, t4255, t4303, t4314, t46292, t67176, t67178, t67180, t67183, t67186, t67191);
        let (t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2311(t152, t20825, t607, t41284, t46302, t20742, t67, t758, t58047, t58052, t58057, t40794);
    (t67195, t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215)
}

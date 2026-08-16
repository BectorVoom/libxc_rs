//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1347;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta241(t2512: f64, t9490: f64, t9720: f64, t761: f64, t9450: f64, t9457: f64, t9463: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9684: f64, t9715: f64, t9718: f64) -> (f64, f64, f64) {
        let t9722 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1347(t2512, t9490, t9720);
        let (t9724, t9725) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1348(t761, t9722, t9450, t9457, t9463, t9469, t9476, t9484, t9496, t9684, t9715, t9718);
    (t9722, t9724, t9725)
}

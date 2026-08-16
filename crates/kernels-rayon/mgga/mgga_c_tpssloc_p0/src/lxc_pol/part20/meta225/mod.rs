//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1304;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta225(t112: f64, t9346: f64, t111: f64, t2311: f64, t2319: f64, t649: f64, t107: f64, t2585: f64, t2281: f64, t667: f64, t2333: f64, t626: f64, t2359: f64, t655: f64, t2332: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9347, t9348) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1304(t112, t9346, t111, t2311);
        let (t9351, t9358, t9359, t9361, t9363, t9364, t9365, t9366) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1305(t2319, t649, t107, t2585, t2281, t667, t2333, t626, t2359, t655, t2332, t666);
    (t9347, t9348, t9351, t9358, t9359, t9361, t9363, t9364, t9365, t9366)
}

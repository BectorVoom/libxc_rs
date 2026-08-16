//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta255(t5392: f64, t9321: f64, t9330: f64, t111: f64, t5449: f64, t5465: f64, t626: f64, t5464: f64, t9365: f64, t5489: f64, t5468: f64, t9384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19420, t19430, t19451, t19471, t19473, t19480, t19488) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk916(t5392, t9321, t9330, t111, t5449, t5465, t626, t5464, t9365, t5489, t5468, t9384);
    (t19420, t19430, t19451, t19471, t19473, t19480, t19488)
}

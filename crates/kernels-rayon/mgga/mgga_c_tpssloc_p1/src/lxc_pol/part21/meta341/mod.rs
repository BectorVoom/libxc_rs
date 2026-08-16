//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1733;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta341(t40: f64, t1409: f64, t9427: f64, t2433: f64, t3966: f64, t12606: f64, t2244: f64, t2250: f64, t4080: f64, t607: f64, t73: f64, t9438: f64, t2440: f64, zeta_threshold: f64, t52: f64, t4087: f64, t76: f64, t157: f64, t182: f64, t145: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12862, t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1733(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
        let (t12886, t12887, t12889, t12890) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1734(t52, t12606, t12874, t12877, t2244, t2250, t4087, t607, t76, t12873, t157, t182, t145, zeta_threshold);
    (t12862, t12874, t12886, t12887, t12889, t12890)
}

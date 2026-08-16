//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2074;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta479(t40: f64, t52: f64, t16549: f64, t16554: f64, t16558: f64, t3966: f64, t4080: f64, t607: f64, t73: f64, t5392: f64, t9438: f64, t2440: f64, t5398: f64, t4087: f64, t76: f64, zeta_threshold: f64, t145: f64, t185: f64, t5520: f64, t751: f64, t157: f64, t182: f64, t12861: f64, t4119: f64, t4315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16562, t16563, t16568, t16574) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2074(t40, t52, t16549, t16554, t16558, t3966, t4080, t607, t73, t5392, t9438, t2440, t5398, t4087, t76, zeta_threshold);
        let (t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2075(t16562, t16574, t145, t185, t5520, t751, t157, t182, t12861, t4119, t4315, t5392);
    (t16563, t16568, t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586)
}

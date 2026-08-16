//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta678(t27553: f64, t95772: f64, t477: f64, t5052: f64, t27654: f64, t7327: f64, t24745: f64, t4935: f64, t24585: f64, t7999: f64, t24574: f64, t27800: f64, t225: f64, t27805: f64, t27392: f64, t1170: f64, t2121: f64, t27766: f64, t2154: f64, t45349: f64, t27776: f64, t11147: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95774, t95794, t95803, t95813, t95824, t95834) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116(t27553, t95772, t477, t5052, t27654, t7327, t24745, t4935, t24585, t7999, t24574, t27800);
        let (t95836, t95863, t95866, t95884, t95889, t95890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2117(t225, t27805, t24574, t27392, t1170, t2121, t27766, t2154, t45349, t27776, t95772, t11147, t497);
    (t95774, t95794, t95803, t95813, t95824, t95834, t95836, t95863, t95866, t95884, t95889, t95890)
}

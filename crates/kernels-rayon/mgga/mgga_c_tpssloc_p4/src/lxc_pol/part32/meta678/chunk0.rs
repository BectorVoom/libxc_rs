//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2116/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116(t27553: f64, t95772: f64, t477: f64, t5052: f64, t27654: f64, t7327: f64, t24745: f64, t4935: f64, t24585: f64, t7999: f64, t24574: f64, t27800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95774 = 0.24369393582936687948e-2_f64 * t95772 * t27553;
    let t95794 = t477 * t5052;
    let t95803 = t27654 * t7327;
    let t95813 = t4935 * t24745;
    let t95824 = t7999 * t24585;
    let t95834 = 0.54831135561607547884e-2_f64 * t24574 * t27800;
    (t95774, t95794, t95803, t95813, t95824, t95834)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2346/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346(t2110: f64, t24517: f64, t26009: f64, t26016: f64, t27298: f64, t27937: f64, t27979: f64, t7256: f64, t7259: f64, t90114: f64, t96102: f64, t96110: f64, t96115: f64, t96120: f64, t96383: f64, t96443: f64, t96646: f64) -> f64 {
    let t104783 = -10.0_f64 / 3.0_f64 * t26016 * t96102 - 10.0_f64 / 3.0_f64 * t26016 * t96110 - 10.0_f64 / 3.0_f64 * t26016 * t96115 + t96646 * t2110 / 3.0_f64 + t27979 * t7256 / 3.0_f64 + t27979 * t7259 / 3.0_f64 - t96383 * t2110 / 6.0_f64 - t27937 * t7256 / 6.0_f64 - t27937 * t7259 / 6.0_f64 - 10.0_f64 * t96120 * t26009 - 10.0_f64 / 3.0_f64 * t90114 * t27298 - 10.0_f64 / 3.0_f64 * t96443 * t24517;
    t104783
}

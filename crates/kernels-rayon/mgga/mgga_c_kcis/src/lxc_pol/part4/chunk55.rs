//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 55/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk55(t60: f64, t127: f64, t129: f64, t130: f64) -> (f64, f64) {
    let t134 = t60 * t60;
    let t136 = 0.19711288999999999999e-2_f64 * t127 * t129 * t130 - 2.0_f64 * t134;
    let t137 = 1.0_f64 / t136;
    (t136, t137)
}

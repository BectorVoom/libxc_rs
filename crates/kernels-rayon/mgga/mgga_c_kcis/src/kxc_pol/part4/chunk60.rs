//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 60/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk60(t122: f64, t144: f64, t145: f64, t148: f64, t85: f64, t137: f64, t113: f64, t62: f64) -> (f64, f64, f64) {
    let t152 = 0.619125e-2_f64 * t144 * t145 - 0.79593333333333333331e-1_f64 * t85 * t148 * t122;
    let t153 = t152 * t137;
    let t154 = t62 * t113;
    (t152, t153, t154)
}

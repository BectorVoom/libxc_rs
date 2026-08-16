//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 281/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk281(t1022: f64, t922: f64, t1021: f64, t1020: f64, t113: f64, t239: f64) -> (f64, f64, f64, f64) {
    let t1023 = t1022 * t922;
    let t1024 = t1021 * t1023;
    let t1025 = t1020 * t1024;
    let t1027 = t113 * t239;
    (t1023, t1024, t1025, t1027)
}

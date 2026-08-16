//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1229/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1229(t100023: f64, t1142: f64, t99903: f64, t99943: f64, t99983: f64, t10498: f64, t1203: f64, t29042: f64, t27987: f64, t5189: f64, t46041: f64, t8064: f64) -> (f64, f64, f64, f64) {
    let t100026 = t1142 * (t99903 + t99943 + t99983 + t100023);
    let t100029 = 6.0_f64 * t10498 * t29042 * t1203;
    let t100031 = 2.0_f64 * t27987 * t5189;
    let t100033 = 4.0_f64 * t46041 * t8064;
    (t100026, t100029, t100031, t100033)
}

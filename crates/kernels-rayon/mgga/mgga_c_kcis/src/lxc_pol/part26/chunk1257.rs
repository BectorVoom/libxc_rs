//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1257/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1257(t1607: f64, t613: f64, t1598: f64, t18256: f64, t251: f64, t18210: f64, t28815: f64, t7968: f64, t27563: f64, t28714: f64, t1370: f64, t7984: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99002 = t613 * t1607;
    let t99013 = t18256 * t251 * t1598;
    let t99023 = t18210 * t28815;
    let t99024 = t7968 * t99023;
    let t99035 = 0.23168402777777777778e-3_f64 * t28714 * t27563;
    let t99046 = t1370 * t7984;
    (t99002, t99013, t99023, t99024, t99035, t99046)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1115/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1115(t14067: f64, t14068: f64, t3200: f64, t1022: f64, t9409: f64, t4818: f64, t922: f64, t2861: f64, t4774: f64, t4549: f64, t9429: f64, t4802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14069 = t14067 * t14068;
    let t14070 = t3200 * t14069;
    let t14072 = t9409 * t1022;
    let t14073 = t4818 * t922;
    let t14074 = t14072 * t14073;
    let t14075 = t3200 * t14074;
    let t14078 = t2861 * t4774;
    let t14079 = 0.33163888888888888888e-2_f64 * t14078;
    let t14081 = t9429 * t4549;
    let t14085 = t9429 * t4802;
    (t14070, t14075, t14078, t14079, t14081, t14085)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1171/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1171(t41: f64, t9545: f64, t14611: f64, t5076: f64, t14110: f64, t5181: f64, t5180: f64, t10799: f64, t1813: f64, t10707: f64, t5062: f64, t3436: f64, t9588: f64) -> (f64, f64, f64, f64, f64) {
    let t14838 = t41 * t9545;
    let t14839 = t14838 * t14611;
    let t14840 = t5076 * t14839;
    let t14842 = t5181 * t14110;
    let t14843 = t5180 * t14842;
    let t14845 = t10799 * t1813;
    let t14847 = t10707 * t5062;
    let t14849 = t9588 * t3436;
    (t14840, t14843, t14845, t14847, t14849)
}

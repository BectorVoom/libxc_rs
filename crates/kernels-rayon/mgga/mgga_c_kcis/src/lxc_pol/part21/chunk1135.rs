//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1135/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1135(t27836: f64, t7719: f64, t1020: f64, t26753: f64, t8047: f64, t167: f64, t3203: f64, t7718: f64, t4994: f64, t1014: f64, t8057: f64, t356: f64, t5013: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27837 = t27836 * t7719;
    let t27838 = t1020 * t27837;
    let t27842 = t26753 * t8047;
    let t27843 = t1020 * t27842;
    let t27845 = t3203 * t167;
    let t27846 = t7718 * t27845;
    let t27847 = t4994 * t27846;
    let t27849 = t1014 * t8057;
    let t27851 = t356 * t5013;
    (t27837, t27838, t27842, t27843, t27845, t27846, t27847, t27849, t27851)
}

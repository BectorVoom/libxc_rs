//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1183/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1183(t14665: f64, t1820: f64, t14668: f64, t5039: f64, t5036: f64, t5189: f64, t10491: f64, t6638: f64, t1203: f64, t10498: f64, t3330: f64, t3325: f64, t6735: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19829 = 2.0_f64 * t14665 * t1820;
    let t19831 = 4.0_f64 * t14668 * t5039;
    let t19833 = 2.0_f64 * t5036 * t5189;
    let t19835 = 2.0_f64 * t10491 * t6638;
    let t19836 = t6638 * t1203;
    let t19838 = 6.0_f64 * t10498 * t19836;
    let t19839 = t1820 * t5189;
    let t19841 = 4.0_f64 * t3330 * t19839;
    let t19842 = t3325 * t6735;
    (t19829, t19831, t19833, t19835, t19838, t19841, t19842)
}

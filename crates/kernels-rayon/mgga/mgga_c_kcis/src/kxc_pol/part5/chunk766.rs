//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 766/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk766(t5875: f64, t5904: f64, t5903: f64, t492: f64, t570: f64, t41: f64, t4134: f64, t4293: f64, t5671: f64, t4292: f64, t5880: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5905 = t5904 * t5875;
    let t5906 = t5903 * t5905;
    let t5908 = t570 * t492;
    let t5909 = t41 * t4134;
    let t5910 = t5909 * t5875;
    let t5911 = t5908 * t5910;
    let t5913 = t4293 * t5671;
    let t5914 = t4292 * t5913;
    let t5916 = t4293 * t5880;
    let t5917 = t4292 * t5916;
    let t5919 = t4261 * t5671;
    (t5905, t5906, t5908, t5909, t5910, t5911, t5913, t5914, t5916, t5917, t5919)
}

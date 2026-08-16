//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 941/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk941(t21860: f64, t4160: f64, t21858: f64, t5426: f64, t15865: f64, t5661: f64, t1363: f64, t7028: f64, t3738: f64, t7037: f64, t4153: f64, t11913: f64, t6905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21861 = t4160 * t21860;
    let t21863 = t5426 * t21858;
    let t21864 = t15865 * t21863;
    let t21865 = t5661 * t21864;
    let t21868 = t7028 * t1363;
    let t21871 = t3738 * t7037;
    let t21872 = t4153 * t21871;
    let t21874 = t11913 * t6905;
    (t21861, t21863, t21865, t21868, t21872, t21874)
}

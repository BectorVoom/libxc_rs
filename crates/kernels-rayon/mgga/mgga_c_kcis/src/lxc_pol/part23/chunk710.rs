//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 710/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk710(t1928: f64, t20: f64, t251: f64, t1592: f64, t1889: f64, t7979: f64, t1600: f64, t2104: f64, t7984: f64, t6176: f64, t2260: f64, t7968: f64, t7976: f64, t7978: f64, t7991: f64, t8166: f64, t8169: f64, t8172: f64, t8177: f64, t8180: f64, t8209: f64, t8213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8217 = t251 * t1928 * t20;
    let t8218 = t1592 * t8217;
    let t8221 = t7979 * t1889;
    let t8222 = t1600 * t8221;
    let t8225 = t7984 * t2104;
    let t8226 = t6176 * t8225;
    let t8236 = -0.34752604166666666667e-3_f64 * t8209 * t2260 + 0.46377350260416666667e-4_f64 * t7968 * t8213 + 0.92673611111111111112e-3_f64 * t8218 * t2260 - t7976 - 0.11584201388888888889e-3_f64 * t7978 * t8222 + 0.34752604166666666667e-3_f64 * t7978 * t8226 + 0.34752604166666666667e-3_f64 * t7978 * t8213 + t7991 + 0.11607361111111111111e-2_f64 * t8166 + 0.17411041666666666666e-2_f64 * t8169 - 0.17411041666666666666e-2_f64 * t8172 - 0.46429444444444444443e-2_f64 * t8177 + 0.11607361111111111111e-2_f64 * t8180;
    (t8217, t8218, t8221, t8222, t8225, t8226, t8236)
}

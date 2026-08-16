//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 887/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk887(t1396: f64, t16665: f64, t4123: f64, t1464: f64, t11914: f64, t1364: f64, t15978: f64, t15987: f64, t15989: f64, t16612: f64, t16615: f64, t16620: f64, t16625: f64, t16628: f64, t16629: f64, t16632: f64, t16636: f64, t16640: f64, t16644: f64, t16651: f64, t16656: f64, t16661: f64, t16663: f64) -> (f64, f64) {
    let t16666 = t1396 * t16665;
    let t16667 = t4123 * t16666;
    let t16668 = t1464 * t16667;
    let t16670 = -t15987 - t15989 - 0.24872916666666666666e-2_f64 * t16612 - 0.55273148148148148147e-3_f64 * t16615 + 0.14739506172839506172e-2_f64 * t16620 + 0.49745833333333333332e-2_f64 * t16625 + t16628 - 0.5895802469135802469e-2_f64 * t16629 - t16632 - 0.73697530864197530861e-3_f64 * t16636 - 0.22109259259259259258e-2_f64 * t16640 - 0.22109259259259259258e-2_f64 * t16644 - 0.22109259259259259258e-2_f64 * t11914 + 0.66725e-1_f64 * t1364 * t15978 + 0.88437037037037037034e-2_f64 * t16651 - 0.16581944444444444444e-2_f64 * t16656 - 0.55273148148148148147e-3_f64 * t16661 - 0.73697530864197530861e-3_f64 * t16663 + 0.99491666666666666664e-2_f64 * t16668;
    (t16668, t16670)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1015/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1015(t1220: f64, t15422: f64, t13714: f64, t10945: f64, t13710: f64, t13712: f64, t13717: f64, t13720: f64, t13723: f64, t13726: f64, t13729: f64, t13732: f64, t13735: f64, t13738: f64, t13742: f64, t9681: f64, t9683: f64, t9691: f64, t9700: f64) -> (f64, f64) {
    let t15423 = t15422 * t1220;
    let t15432 = 0.2283111111111111111e-1_f64 * t13714;
    let t15442 = -t10945 - 0.1522074074074074074e-1_f64 * t9691 + 0.38051851851851851851e-2_f64 * t9683 - 0.11415555555555555555e-1_f64 * t9700 + 0.57077777777777777777e-2_f64 * t9681 - 0.76103703703703703702e-2_f64 * t13710 + 0.76103703703703703701e-2_f64 * t13712 - t15432 + 0.1255711111111111111e0_f64 * t13717 - 0.19025925925925925925e-1_f64 * t13720 + 0.68493333333333333331e-1_f64 * t13723 - 0.45662222222222222221e-1_f64 * t13726 - 0.11415555555555555555e-1_f64 * t13729 - 0.10274e0_f64 * t13732 + 0.13698666666666666666e0_f64 * t13735 + 0.34246666666666666666e-1_f64 * t13738 - 0.34246666666666666666e-1_f64 * t13742;
    (t15423, t15442)
}

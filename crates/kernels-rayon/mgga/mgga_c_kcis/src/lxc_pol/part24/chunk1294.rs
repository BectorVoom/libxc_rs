//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1294/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1294(t101064: f64, t4947: f64, t93508: f64, t14447: f64, t29010: f64, t7703: f64, t18476: f64, t922: f64, t100486: f64, t100489: f64, t100494: f64, t100505: f64, t100514: f64, t100519: f64, t14492: f64, t19396: f64, t93485: f64, t95903: f64, t95938: f64) -> (f64, f64, f64) {
    let t101250 = t4947 * t93508 * t101064;
    let t101264 = t7703 * t14447 * t29010;
    let t101271 = t4947 * t18476 * t922;
    let t101281 = -0.10297067901234567901e-3_f64 * t101264 + 0.18424382716049382715e-2_f64 * t100486 - 0.73697530864197530861e-2_f64 * t100489 - 0.22109259259259259259e-2_f64 * t95903 + 0.14739506172839506172e-2_f64 * t100494 + 0.23168402777777777778e-3_f64 * t7703 * t101271 - 0.16581944444444444444e-2_f64 * t100505 - 0.72079475308641975309e-3_f64 * t7703 * t14492 * t93485 * t19396 + t95938 - 0.22109259259259259259e-2_f64 * t100514 - 0.33163888888888888888e-2_f64 * t100519;
    (t101250, t101271, t101281)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 923/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk923(t1131: f64, t19763: f64, t1096: f64, t1092: f64, t2855: f64, t6330: f64, t1021: f64, t1020: f64, t10450: f64, t13399: f64, t13409: f64, t14065: f64, t14079: f64, t14081: f64, t14086: f64, t14103: f64, t14104: f64, t14390: f64, t19107: f64, t19738: f64, t19743: f64, t19747: f64, t19752: f64, t19754: f64, t19759: f64, t300: f64) -> (f64, f64, f64) {
    let t19764 = t1131 * t19763;
    let t19765 = t1096 * t19764;
    let t19766 = t1092 * t19765;
    let t19769 = t2855 * t6330;
    let t19770 = t1021 * t19769;
    let t19771 = t1020 * t19770;
    let t19775 = 0.44218518518518518516e-2_f64 * t13399 + t13409 - 0.16581944444444444444e-2_f64 * t19738 + 0.11054629629629629629e-2_f64 * t19743 + 0.18424382716049382715e-2_f64 * t19747 - 0.11054629629629629629e-2_f64 * t14065 + 0.66327777777777777776e-2_f64 * t19752 + 0.16581944444444444444e-2_f64 * t19754 - t14079 - 0.55273148148148148147e-3_f64 * t19759 - 0.44218518518518518516e-2_f64 * t14081 - t14086 + t14103 - 0.22109259259259259259e-2_f64 * t14104 - 0.16581944444444444444e-2_f64 * t19766 + t19107 * t300 + 0.88437037037037037035e-2_f64 * t19771 - 0.55273148148148148147e-3_f64 * t10450 - 0.7369753086419753086e-3_f64 * t14390;
    (t19766, t19771, t19775)
}

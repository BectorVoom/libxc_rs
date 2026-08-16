//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 862/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk862(t1947: f64, t5752: f64, t1394: f64, t1396: f64, t6944: f64, t1395: f64, t6937: f64, t4153: f64, t1943: f64, t3717: f64, t1944: f64, t3961: f64, t507: f64, t5681: f64, t5684: f64, t5742: f64, t6906: f64, t6910: f64, t6915: f64, t6920: f64, t6925: f64, t6930: f64, t6934: f64, t7028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7030 = t5752 * t1947;
    let t7031 = t1394 * t7030;
    let t7033 = t1396 * t6944;
    let t7034 = t1395 * t7033;
    let t7035 = t1394 * t7034;
    let t7037 = t1396 * t6937;
    let t7038 = t1395 * t7037;
    let t7039 = t4153 * t7038;
    let t7042 = t1943 * t1943;
    let t7043 = t7042 * t3717;
    let t7049 = 0.22109259259259259258e-2_f64 * t6906 - 0.88437037037037037034e-2_f64 * t6910 - 0.33163888888888888888e-2_f64 * t6915 - 0.33163888888888888888e-2_f64 * t6920 - 0.55273148148148148147e-3_f64 * t6925 + 0.49745833333333333332e-2_f64 * t6930 + 0.13265555555555555555e-1_f64 * t6934 + t7028 * t507 + 0.33163888888888888888e-2_f64 * t7031 + 0.16581944444444444444e-2_f64 * t7035 + 0.27636574074074074073e-2_f64 * t7039 + 0.22109259259259259258e-2_f64 * t5681 + 0.890445125e-2_f64 * t3961 * t7043 - 0.13345e0_f64 * t5742 * t1944 - 0.88437037037037037034e-2_f64 * t5684;
    (t7030, t7031, t7033, t7034, t7035, t7037, t7038, t7039, t7042, t7043, t7049)
}

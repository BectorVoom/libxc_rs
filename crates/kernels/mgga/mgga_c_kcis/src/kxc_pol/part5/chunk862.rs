//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 862/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk862<F: Float>(t1947: F, t5752: F, t1394: F, t1396: F, t6944: F, t1395: F, t6937: F, t4153: F, t1943: F, t3717: F, t1944: F, t3961: F, t507: F, t5681: F, t5684: F, t5742: F, t6906: F, t6910: F, t6915: F, t6920: F, t6925: F, t6930: F, t6934: F, t7028: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
    let t7049 = F::cast_from(0.22109259259259259258e-2_f64) * t6906 - F::cast_from(0.88437037037037037034e-2_f64) * t6910 - F::cast_from(0.33163888888888888888e-2_f64) * t6915 - F::cast_from(0.33163888888888888888e-2_f64) * t6920 - F::cast_from(0.55273148148148148147e-3_f64) * t6925 + F::cast_from(0.49745833333333333332e-2_f64) * t6930 + F::cast_from(0.13265555555555555555e-1_f64) * t6934 + t7028 * t507 + F::cast_from(0.33163888888888888888e-2_f64) * t7031 + F::cast_from(0.16581944444444444444e-2_f64) * t7035 + F::cast_from(0.27636574074074074073e-2_f64) * t7039 + F::cast_from(0.22109259259259259258e-2_f64) * t5681 + F::cast_from(0.890445125e-2_f64) * t3961 * t7043 - F::new(0.13345e0) * t5742 * t1944 - F::cast_from(0.88437037037037037034e-2_f64) * t5684;
    (t7030, t7031, t7033, t7034, t7035, t7037, t7038, t7039, t7042, t7043, t7049)
}

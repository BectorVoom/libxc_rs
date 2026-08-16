//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 571/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk571(t1254: f64, t7959: f64, t4083: f64, t7927: f64, t4087: f64, t6020: f64, t7914: f64, t7917: f64, t7920: f64, t2141: f64, t1275: f64, t4100: f64) -> (f64, f64, f64, f64, f64) {
    let t7960 = t7959 * t1254;
    let t7963 = t7927 * t4083;
    let t7970 = t4087 + 0.61805555555555555556e-2_f64 * t6020 - 0.61805555555555555555e-2_f64 * t7914 + 0.18541666666666666667e-1_f64 * t7917 - 0.92708333333333333333e-2_f64 * t7920;
    let t7976 = t2141 * t2141;
    let t7978 = t4100 * t7976 * t1275;
    (t7960, t7963, t7970, t7976, t7978)
}

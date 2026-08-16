//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1178/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1178(t1745: f64, t2844: f64, t2630: f64, t3399: f64, t11: f64, t41: f64, t85: f64, t5143: f64, t5135: f64, t10552: f64, t10554: f64, t10556: f64, t10558: f64, t10599: f64, t1153: f64, t14118: f64, t14242: f64, t14287: f64, t14922: f64, t14926: f64, t14927: f64, t14930: f64, t14940: f64, t14944: f64, t2429: f64, t3381: f64, t5122: f64) -> (f64, f64) {
    let t14947 = t1745 * t2844;
    let t14949 = t3399 * t14947 * t2630;
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t14956 = t14955 * t5143;
    let t14959 = 0.5895802469135802469e-1_f64 * t14955 * t5135;
    let t14960 = -0.10612444444444444444e0_f64 * t2429 * t14922 - t14926 + 0.88437037037037037036e-1_f64 * t14927 - 0.26531111111111111111e-1_f64 * t1153 * t14930 - 0.17687407407407407407e-1_f64 * t10552 - 0.29479012345679012345e-1_f64 * t10554 + 0.11791604938271604938e-1_f64 * t10556 - 0.35374814814814814814e-1_f64 * t10558 + 0.35374814814814814814e-1_f64 * t10599 - 0.9286875e-2_f64 * t3381 * t14242 - 0.232171875e-2_f64 * t14940 * t14118 - 0.26531111111111111111e-1_f64 * t1153 * t14944 - 0.44218518518518518518e-1_f64 * t1153 * t14949 + 0.123825e-1_f64 * t5122 * t14287 + 0.70749629629629629629e-1_f64 * t14956 - t14959;
    (t14955, t14960)
}

//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1014/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1014(t1587: f64, t398: f64, t1591: f64, t4376: f64, t1586: f64, t4374: f64, t1163: f64, t1312: f64, t1390: f64, t1588: f64, t14910: f64, t1572: f64, t4420: f64) -> (f64, f64, f64, f64) {
    let t14961 = t1587 * t1587;
    let t14962 = 1.0_f64 / t14961;
    let t14963 = t398 * t14962;
    let t14964 = t4376 * t1591;
    let t14965 = t14963 * t14964;
    let t14966 = t1586 * t14965;
    let t14971 = t4374 * t4376;
    let t14972 = t14971 * t1163;
    let t14973 = t1312 * t14972;
    let t14978 = t1588 * t1390;
    let t14979 = t14978 * t14910;
    let t14980 = t1312 * t14979;
    let t14983 = t1572 * t4420;
    (t14966, t14973, t14980, t14983)
}

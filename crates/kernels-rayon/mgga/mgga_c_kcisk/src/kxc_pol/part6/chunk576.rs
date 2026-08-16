//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 576/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk576(t3952: f64, t8032: f64, t2075: f64, t2168: f64, t3937: f64, t3942: f64, t7706: f64, t1312: f64, t1313: f64, t7710: f64, t3960: f64, t1310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8033 = t3952 * t8032;
    let t8036 = t2075 * t2168;
    let t8037 = t3937 * t8036;
    let t8040 = t3942 * t7706;
    let t8041 = t1312 * t8040;
    let t8044 = t1313 * t7710;
    let t8045 = t1312 * t8044;
    let t8048 = t2168 * t2168;
    let t8049 = t3960 * t8048;
    let t8050 = t1310 * t8049;
    (t8033, t8036, t8037, t8040, t8041, t8044, t8045, t8048, t8049, t8050)
}

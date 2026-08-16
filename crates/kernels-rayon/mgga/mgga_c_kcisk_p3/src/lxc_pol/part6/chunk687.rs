//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 687/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk687(t10463: f64, t702: f64, t11226: f64, t740: f64, t5438: f64, t791: f64, t10501: f64, t1992: f64, t772: f64, t10568: f64, t5396: f64, t760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11911 = t702 * t10463;
    let t11927 = t11226 * t740;
    let t11966 = 1.0_f64 / t5438 / t791;
    let t11983 = 0.51588271604938271604e-3_f64 * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = 1.0_f64 / t11984;
    let t11986 = t772 * t11985;
    let t12002 = 0.53272592592592592592e-1_f64 * t10568;
    let t12017 = 1.0_f64 / t5396 / t760;
    (t11911, t11927, t11966, t11983, t11986, t12002, t12017)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1045/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1045(t1415: f64, t31147: f64, t1411: f64, t1220: f64, t26869: f64, t30738: f64, t30956: f64, t30960: f64, t30965: f64, t30969: f64, t30975: f64, t30980: f64, t30984: f64, t30988: f64, t30992: f64, t30994: f64, t31002: f64, t31136: f64, t31139: f64, t31144: f64, t3930: f64, t412: f64) -> (f64, f64) {
    let t31148 = t1415 * t31147;
    let t31149 = t1411 * t31148;
    let t31151 = 0.8290972222222222222e-2_f64 * t30956 + 0.99491666666666666664e-2_f64 * t30960 + 0.16581944444444444444e-2_f64 * t30965 - 0.16581944444444444444e-1_f64 * t30969 + t30738 * t412 - 0.49745833333333333332e-2_f64 * t30975 + 0.33163888888888888887e-2_f64 * t30980 - 0.99491666666666666664e-2_f64 * t30984 + 0.82909722222222222219e-2_f64 * t30988 - 0.8290972222222222222e-2_f64 * t30992 + 0.579e0_f64 * t1220 * t30994 + 0.223494e0_f64 * t3930 * t30994 - 0.99491666666666666664e-2_f64 * t26869 - 0.66327777777777777775e-2_f64 * t31002 - 0.24872916666666666666e-2_f64 * t31136 - 0.99491666666666666664e-2_f64 * t31139 + 0.1492375e-1_f64 * t31144 - 0.49745833333333333332e-2_f64 * t31149;
    (t31149, t31151)
}

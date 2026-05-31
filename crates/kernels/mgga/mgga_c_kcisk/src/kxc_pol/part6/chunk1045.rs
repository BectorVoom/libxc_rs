//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1045/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1045<F: Float>(t1415: F, t31147: F, t1411: F, t1220: F, t26869: F, t30738: F, t30956: F, t30960: F, t30965: F, t30969: F, t30975: F, t30980: F, t30984: F, t30988: F, t30992: F, t30994: F, t31002: F, t31136: F, t31139: F, t31144: F, t3930: F, t412: F) -> (F, F) {
    let t31148 = t1415 * t31147;
    let t31149 = t1411 * t31148;
    let t31151 = F::cast_from(0.8290972222222222222e-2_f64) * t30956 + F::cast_from(0.99491666666666666664e-2_f64) * t30960 + F::cast_from(0.16581944444444444444e-2_f64) * t30965 - F::cast_from(0.16581944444444444444e-1_f64) * t30969 + t30738 * t412 - F::cast_from(0.49745833333333333332e-2_f64) * t30975 + F::cast_from(0.33163888888888888887e-2_f64) * t30980 - F::cast_from(0.99491666666666666664e-2_f64) * t30984 + F::cast_from(0.82909722222222222219e-2_f64) * t30988 - F::cast_from(0.8290972222222222222e-2_f64) * t30992 + F::cast_from(0.579e0_f64) * t1220 * t30994 + F::cast_from(0.223494e0_f64) * t3930 * t30994 - F::cast_from(0.99491666666666666664e-2_f64) * t26869 - F::cast_from(0.66327777777777777775e-2_f64) * t31002 - F::cast_from(0.24872916666666666666e-2_f64) * t31136 - F::cast_from(0.99491666666666666664e-2_f64) * t31139 + F::cast_from(0.1492375e-1_f64) * t31144 - F::cast_from(0.49745833333333333332e-2_f64) * t31149;
    (t31149, t31151)
}

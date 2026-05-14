//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 924/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk924<F: Float>(t1415: F, t31134: F, t1411: F, t5606: F, t8180: F, t1339: F, t2231: F, t7906: F, t1341: F, t3785: F, t2152: F, t1450: F, t1220: F, t26869: F, t30738: F, t30956: F, t30960: F, t30965: F, t30969: F, t30975: F, t30980: F, t30984: F, t30988: F, t30992: F, t30994: F, t31002: F, t3930: F, t412: F) -> (F, F, F, F, F, F, F) {
    let t31135 = t1415 * t31134;
    let t31136 = t1411 * t31135;
    let t31138 = t5606 * t8180;
    let t31139 = t1339 * t31138;
    let t31141 = t7906 * t2231;
    let t31142 = t1341 * t31141;
    let t31143 = t3785 * t31142;
    let t31144 = t1411 * t31143;
    let t31146 = t7906 * t2152;
    let t31147 = t1450 * t31146;
    let t31148 = t1415 * t31147;
    let t31149 = t1411 * t31148;
    let t31151 = 0.8290972222222222222e-2 * t30956 + 0.99491666666666666664e-2 * t30960 + 0.16581944444444444444e-2 * t30965 - 0.16581944444444444444e-1 * t30969 + t30738 * t412 - 0.49745833333333333332e-2 * t30975 + 0.33163888888888888887e-2 * t30980 - 0.99491666666666666664e-2 * t30984 + 0.82909722222222222219e-2 * t30988 - 0.8290972222222222222e-2 * t30992 + 0.579e0 * t1220 * t30994 + 0.223494e0 * t3930 * t30994 - 0.99491666666666666664e-2 * t26869 - 0.66327777777777777775e-2 * t31002 - 0.24872916666666666666e-2 * t31136 - 0.99491666666666666664e-2 * t31139 + 0.1492375e-1 * t31144 - 0.49745833333333333332e-2 * t31149;
    (t31136, t31139, t31141, t31144, t31146, t31149, t31151)
}

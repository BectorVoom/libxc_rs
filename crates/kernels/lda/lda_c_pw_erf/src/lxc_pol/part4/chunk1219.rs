//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1219/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1219<F: Float>(t10043: F, t10092: F, t10098: F, t10115: F, t13585: F, t13587: F, t13589: F, t16336: F, t16338: F, t16341: F, t16345: F, t16348: F, t10090: F, t16374: F, t16389: F, t16395: F, t16397: F, t16399: F, t16402: F, t16410: F, t16416: F, t16432: F, t16434: F, t16437: F) -> (F, F) {
    let t18077 = -0.003778333333333333 * t16336 + 0.01847185185185185 * t16338 - 0.005037777777777778 * t16341 - 0.0016792592592592592 * t16345 + 0.015113333333333333 * t16348 - t10043 - 0.0008396296296296296 * t13585 - 0.0013993827160493828 * t13587 + 0.0033585185185185185 * t13589 - 0.0016792592592592592 * t10092 + 0.000559753086419753 * t10098 + 0.0008396296296296296 * t10115;
    let t18091 = -0.0012594444444444445 * t16374 + 0.010075555555555556 * t16389 - 0.030226666666666666 * t16395 + 0.0008396296296296296 * t16397 + 0.000559753086419753 * t16399 - 0.09068 * t16402 + 0.06045333333333333 * t16410 - 0.01679259259259259 * t16416 - 0.003918271604938271 * t10090 - 0.007556666666666666 * t16432 - 0.05541555555555556 * t16434 + 0.011335 * t16437;
    (t18077, t18091)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1220/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1220<F: Float>(t13708: F, t13710: F, t13712: F, t13714: F, t13722: F, t13724: F, t13726: F, t13731: F, t13736: F, t16486: F, t16489: F, t16492: F, t16495: F, t18065: F, t18077: F, t18091: F, t184: F, t203: F, t221: F) -> (F,) {
    let t18105 = -0.007556666666666666 * t16486 - 0.02518888888888889 * t16489 + 0.002099074074074074 * t16492 + 0.005597530864197531 * t16495 + 0.002518888888888889 * t13708 - 0.006717037037037037 * t13710 - 0.007556666666666666 * t13712 + 0.002239012345679012 * t13714 + 0.002518888888888889 * t13722 + 0.005037777777777778 * t13724 - 0.010075555555555556 * t13726 - 0.003918271604938271 * t13731 - 0.059613703703703703 * t13736;
    let t18111 = 2.0 / 15.0 * t203 * (t18065 + t18077 + t18091 + t18105) * t184 * t221;
    (t18111,)
}

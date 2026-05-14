//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1218/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1218<F: Float>(t11907: F, t13771: F, t15728: F, t13035: F, t6759: F, t6763: F, t1394: F, t2443: F, t16276: F, t16280: F, t16292: F, t16295: F, t16297: F, t16302: F, t16308: F, t16311: F, t16314: F, t16325: F, t16327: F, t16332: F) -> (F, F, F, F, F) {
    let t18046 = 64.0 / 9.0 * t13771 * t11907 * t15728;
    let t18048 = 32.0 / 45.0 * t13035 * t6759;
    let t18050 = 64.0 / 45.0 * t13035 * t6763;
    let t18052 = 4.0 / 15.0 * t2443 * t1394;
    let t18065 = -0.04534 * t16276 + 0.011335 * t16280 + 0.005037777777777778 * t16292 + 0.04534 * t16295 - 0.0013993827160493828 * t16297 - 0.007556666666666666 * t16302 + 0.002518888888888889 * t16308 + 0.0012594444444444445 * t16311 + 0.002099074074074074 * t16314 + 0.002518888888888889 * t16325 - 0.0008396296296296296 * t16327 - 0.007556666666666666 * t16332;
    (t18046, t18048, t18050, t18052, t18065)
}

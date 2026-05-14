//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1056/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1056<F: Float>(t50: F, t34: F, t352: F, t462: F, t1063: F, t11456: F, t1789: F, t2334: F, t2337: F, t2849: F, t2966: F, t35: F, t4367: F, t5997: F, t6002: F, t6005: F, t8334: F, t950: F, t951: F, t954: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t15381 = t352 * t34 * t462;
    let t15402 = piecewise3(t51, 0.0, 40.0 / 81.0 * t8334 * t2334 * t951 + 64.0 / 27.0 * t4367 * t15381 - 8.0 / 27.0 * t5997 * t954 + 32.0 / 9.0 * t950 * t35 * t1063 - 16.0 / 9.0 * t1789 * t462 + 16.0 / 3.0 * t1789 * t2849 - 8.0 / 27.0 * t2966 * t2337 * t951 + 8.0 / 9.0 * t950 * t6005 * t352 + 4.0 / 9.0 * t6002 * t954 - t11456);
    (t15381, t15402)
}

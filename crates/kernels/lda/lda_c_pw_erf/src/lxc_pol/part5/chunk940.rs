//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 940/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk940<F: Float>(t50: F, t17673: F, t1789: F, t20019: F, t20027: F, t352: F, t4367: F, t4370: F, t52: F, t5997: F, t6005: F, t7365: F, t7370: F, t8334: F, t943: F, t950: F, t20011: F, t59: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t20031 = piecewise3(t51, 0.0, 40.0 / 81.0 * t8334 * t7365 * t352 + 16.0 / 9.0 * t5997 * t943 - 8.0 / 9.0 * t4367 * t17673 - 8.0 / 3.0 * t4370 * t20019 + 4.0 / 3.0 * t1789 * t6005 + 4.0 / 9.0 * t950 * t7370 * t352 + 4.0 / 3.0 * t52 * t20027);
    let t20033 = (t20011 + t20031) * t59;
    (t20033,)
}

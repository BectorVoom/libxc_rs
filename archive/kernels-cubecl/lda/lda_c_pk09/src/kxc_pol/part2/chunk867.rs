//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 867/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk867<F: Float>(t4095: F, t8990: F, t1101: F, t3105: F, t3107: F, t4085: F, t4096: F, t4109: F, t4111: F, t4123: F, t4125: F, t4128: F, t709: F, t8096: F, t8101: F, t8977: F, t8980: F, t8987: F) -> F {
    let t8991 = t8990 * t4095;
    let t8998 = -t8977 * t709 / F::cast_from(6.0_f64) - t8980 / F::cast_from(6.0_f64) - t1101 * t8096 / F::cast_from(6.0_f64) - t1101 * t8101 / F::cast_from(6.0_f64) + t4085 * t8987 / F::cast_from(36.0_f64) - t8991 / F::cast_from(18.0_f64) - t4096 / F::cast_from(36.0_f64) - t4109 + t4111 / F::cast_from(6.0_f64) + t4123 / F::cast_from(6.0_f64) - t4125 - t4128 + F::cast_from(0.016445729887122652_f64) * t3105 + F::cast_from(0.016445729887122652_f64) * t3107;
    t8998
}

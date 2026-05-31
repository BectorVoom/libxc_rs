//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 663/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk663<F: Float>(t1337: F, t6030: F, t142: F, t1524: F, t3248: F, t5164: F, t1336: F, t5009: F, t409: F, t5156: F, t1438: F, t378: F) -> (F, F, F, F, F, F, F) {
    let t6031 = t1337 * t6030;
    let t6033 = t1524 * t142;
    let t6035 = t6033 * t3248 * t5164;
    let t6037 = t1336 * t142;
    let t6041 = t5009 * t142;
    let t6043 = t6041 * t409 * t5156;
    let t6050 = t1438 * t1438;
    let t6052 = F::cast_from(1.0_f64) / t6050 / t378;
    (t6031, t6033, t6035, t6037, t6041, t6043, t6052)
}

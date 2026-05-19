//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 878/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk878<F: Float>(t7831: F, t96: F, t93: F, t1067: F, t2363: F, t1101: F, t8141: F, t1076: F, t7991: F, t2355: F, t1011: F, t2210: F, t4342: F, t4343: F, t4440: F, t7768: F, t7962: F, t8121: F, t90: F, t9043: F, t9046: F) -> (F, F) {
    let t9158 = t96 * t7831;
    let t9159 = t93 * t9158;
    let t9171 = t2363 * t1067;
    let t9173 = t1101 * t8141;
    let t9175 = t1076 * t7991;
    let t9177 = t2355 * t1067;
    let t9179 = t1076 * t8141;
    let t9181 = -t1076 * t9043 / F::new(3.0) - t1076 * t9046 / F::new(6.0) - t90 * t9159 / F::new(6.0) + t4342 + t4343 - F::cast_from(0.14975624337724558_f64) * t8121 - t2363 * t1011 / F::new(6.0) - t4440 * t2210 / F::new(6.0) - t1101 * t7962 / F::new(6.0) - t1101 * t7768 / F::new(6.0) + t9171 / F::new(9.0) + t9173 / F::new(9.0) - t9175 / F::new(9.0) + t9177 / F::new(9.0) - t9179 / F::new(9.0);
    (t9159, t9181)
}

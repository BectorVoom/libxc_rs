//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 448/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk448<F: Float>(t1000: F, t1001: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t995: F, t996: F, t101: F, t89: F) -> (F, F, F) {
    let t2392 = t995 + t996 + F::cast_from(2.2984542076810275_f64) * t2159 + F::cast_from(2.2984542076810275_f64) * t2163 - F::cast_from(2.2984542076810275_f64) * t2167 + t1000 + t1001 + F::cast_from(0.15282509383508946_f64) * t2171 + F::cast_from(0.15282509383508946_f64) * t2175 - F::cast_from(0.15282509383508946_f64) * t2179;
    let t2393 = t101 * t2392;
    let t2394 = t2393 * t89;
    (t2392, t2393, t2394)
}

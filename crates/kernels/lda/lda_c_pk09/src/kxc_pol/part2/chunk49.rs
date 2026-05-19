//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 49/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk49<F: Float>(t79: F, t66: F, t77: F, t89: F) -> (F, F, F, F) {
    let t101 = F::ln(t79);
    let t104 = F::cast_from(1.1492271038405137_f64) * t66 + F::cast_from(0.15282509383508946_f64) * t77 + F::cast_from(0.01795667349750801_f64);
    let t105 = t101 * t104;
    let t106 = t105 * t89;
    (t101, t104, t105, t106)
}

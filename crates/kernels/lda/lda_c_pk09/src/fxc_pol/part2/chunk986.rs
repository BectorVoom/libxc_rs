//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 986/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk986<F: Float>(t10555: F, t10571: F, t10533: F, t10535: F, t10540: F, t1397: F, t1417: F, t2621: F, t392: F, t5139: F, t5144: F, t1425: F) -> (F, F) {
    let t10572 = t10555 + t10571;
    let t10575 = t10533 * t392 - t10535 * t1397 / F::new(2.0) - t5139 * t2621 / F::new(2.0) + F::new(3.0) / F::new(4.0) * t5144 * t10540 - t1417 * t10572 / F::new(2.0);
    let t10576 = t10575 * t1425;
    (t10572, t10576)
}

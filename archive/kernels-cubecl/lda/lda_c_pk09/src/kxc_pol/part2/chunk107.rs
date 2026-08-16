//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 107/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk107<F: Float>(t296: F, t287: F, t294: F) -> (F, F, F) {
    let t314 = F::ln(t296);
    let t317 = F::cast_from(0.38307570128017127_f64) * t287 + F::cast_from(0.15282509383508946_f64) * t294 + F::cast_from(0.01795667349750801_f64);
    let t318 = t314 * t317;
    (t314, t317, t318)
}

//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1082/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1082<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t7192: F, t7199: F, t7200: F, t7205: F) -> F {
    let t11834 = F::new(1.532302805120685) * t6527 - F::new(1.532302805120685) * t6519 - F::new(0.7661514025603425) * t11556 + F::new(0.7661514025603425) * t11559 - F::new(0.05094169794502982) * t10962 + F::new(0.510767601706895) * t11563 - F::new(0.510767601706895) * t11566 - F::new(0.15282509383508946) * t11070 - F::new(0.15282509383508946) * t10954 - F::new(0.15282509383508946) * t10966 - F::new(0.15282509383508946) * t11062 + F::new(0.7661514025603425) * t11574 + F::new(0.15282509383508946) * t6327 + t7192 + t7199 - t7200 - t7205;
    t11834
}

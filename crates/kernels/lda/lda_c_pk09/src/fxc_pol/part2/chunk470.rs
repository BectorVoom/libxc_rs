//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 470/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk470<F: Float>(t1571: F, t1573: F, t1575: F, t1577: F, t2502: F, t2505: F, t2542: F, t2546: F, t314: F, t306: F, t2487: F, t318: F) -> (F, F, F, F) {
    let t2578 = t1571 - F::new(0.7661514025603425) * t2542 + t1573 + F::new(0.7661514025603425) * t2546 + t1575 - F::new(0.15282509383508946) * t2502 + t1577 + F::new(0.15282509383508946) * t2505;
    let t2579 = t314 * t2578;
    let t2580 = t2579 * t306;
    let t2583 = t318 * t2487;
    (t2578, t2579, t2580, t2583)
}

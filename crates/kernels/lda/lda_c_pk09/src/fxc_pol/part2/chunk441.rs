//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 441/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk441<F: Float>(t2578: F, t314: F, t306: F, t2487: F, t318: F, t1349: F, t2143: F, t309: F) -> (F, F, F, F) {
    let t2579 = t314 * t2578;
    let t2580 = t2579 * t306;
    let t2583 = t318 * t2487;
    let t2587 = t309 * t1349 * t2143;
    (t2579, t2580, t2583, t2587)
}

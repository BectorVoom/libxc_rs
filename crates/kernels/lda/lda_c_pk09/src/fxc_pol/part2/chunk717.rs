//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 717/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk717<F: Float>(t151: F, t7991: F, t143: F, t3277: F, t3287: F, t3292: F, t3300: F, t4061: F, t4072: F, t4581: F, t7578: F, t7590: F, t7598: F, t7602: F, t7962: F, t7974: F, t7981: F, t7989: F) -> (F,) {
    let t7992 = t151 * t7991;
    let t7997 = 3.7610742193750633 * t143 * t7962 + 2.2140749178833072 * t7974 * t4061 + 18.635258017632964 * t4581 * t7590 + 37.27051603526593 * t4581 * t7578 - 2.2140749178833072 * t7981 - 4.4281498357666145 * t4072 * t7598 - 2.2140749178833072 * t4072 * t7602 + 0.04115066352984959 * t7989 + 1.2536914064583544 * t7992 + 1.6183441301295518 * t3277 + 2.427516195194328 * t3287 + t3292 - 1.8805371096875316 * t3300;
    (t7997,)
}

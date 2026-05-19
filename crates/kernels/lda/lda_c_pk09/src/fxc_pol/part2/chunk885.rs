//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 885/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk885<F: Float>(t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> F {
    let t9328 = F::cast_from(0.5892551084779716_f64) * t7896 - F::cast_from(8.862261095289186_f64) * t7900 - F::cast_from(8.862261095289186_f64) * t7904 - F::cast_from(8.862261095289186_f64) * t7908 + F::cast_from(13.29339164293378_f64) * t7913 + F::cast_from(8.862261095289186_f64) * t7917 + F::cast_from(0.2946275542389858_f64) * t7919 + F::cast_from(0.2946275542389858_f64) * t7923 + F::cast_from(0.2946275542389858_f64) * t7926 + F::cast_from(0.2946275542389858_f64) * t7928 + F::cast_from(0.2946275542389858_f64) * t7931 + F::cast_from(0.2946275542389858_f64) * t7935 + F::cast_from(0.1964183694926572_f64) * t7939 + F::cast_from(0.1964183694926572_f64) * t7942;
    t9328
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 713/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk713<F: Float>(t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F, t7848: F, t7864: F, t7893: F) -> (F,) {
    let t7944 = 2.4933892525089543 * t7896 - 37.5 * t7900 - 37.5 * t7904 - 37.5 * t7908 + 56.25 * t7913 + 37.5 * t7917 + 1.2466946262544771 * t7919 + 1.2466946262544771 * t7923 + 1.2466946262544771 * t7926 + 1.2466946262544771 * t7928 + 1.2466946262544771 * t7931 + 1.2466946262544771 * t7935 + 0.8311297508363181 * t7939 + 0.8311297508363181 * t7942;
    let t7946 = t7848 + t7864 + t7893 + t7944;
    (t7946,)
}

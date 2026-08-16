//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 816/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk816(t7896: f64, t7900: f64, t7904: f64, t7908: f64, t7913: f64, t7917: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64) -> f64 {
    let t8229 = 1.011531678467958_f64 * t7896 - 12.0_f64 * t7900 - 12.0_f64 * t7904 - 12.0_f64 * t7908 + 18.0_f64 * t7913 + 12.0_f64 * t7917 + 0.505765839233979_f64 * t7919 + 0.505765839233979_f64 * t7923 + 0.505765839233979_f64 * t7926 + 0.505765839233979_f64 * t7928 + 0.505765839233979_f64 * t7931 + 0.505765839233979_f64 * t7935 + 0.337177226155986_f64 * t7939 + 0.337177226155986_f64 * t7942;
    t8229
}

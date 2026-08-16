//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 863/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk863(t7900: f64, t7917: f64, t7896: f64, t7904: f64, t7908: f64, t7913: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64) -> f64 {
    let t8929 = 24.0_f64 * t7900;
    let t8933 = 24.0_f64 * t7917;
    let t8942 = 1.642838787112742_f64 * t7896 - t8929 - 24.0_f64 * t7904 - 24.0_f64 * t7908 + 36.0_f64 * t7913 + t8933 + 0.821419393556371_f64 * t7919 + 0.821419393556371_f64 * t7923 + 0.821419393556371_f64 * t7926 + 0.821419393556371_f64 * t7928 + 0.821419393556371_f64 * t7931 + 0.821419393556371_f64 * t7935 + 0.5476129290375806_f64 * t7939 + 0.5476129290375806_f64 * t7942;
    t8942
}

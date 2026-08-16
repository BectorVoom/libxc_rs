//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 135/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk135(t44: f64, t51: f64, t423: f64, t424: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t425 = f64::powf(t51, t423);
    let t426 = piecewise3(t52, t424, t425);
    let t427 = f64::powf(t44, t423);
    let t428 = piecewise3(t45, t424, t427);
    let t429 = t426 + t428;
    (t425, t427, t429)
}

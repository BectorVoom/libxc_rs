//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 852/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk852(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t3629: f64, t3632: f64, t3633: f64, t3634: f64, t7851: f64, t7855: f64) -> f64 {
    let t8785 = 9.625452574583042_f64 * t7851 + 9.625452574583042_f64 * t7855 - 0.64_f64 * t3335 - 0.4266666666666667_f64 * t3342 + 19.250905149166083_f64 * t3384 + 19.250905149166083_f64 * t3388 - 19.250905149166083_f64 * t3393 + t3629 + t3632 + t3633 - t3634 + 0.64_f64 * t3317 + 0.64_f64 * t3319;
    t8785
}

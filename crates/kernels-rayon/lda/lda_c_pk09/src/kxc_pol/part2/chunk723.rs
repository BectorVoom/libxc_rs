//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 723/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk723(t1468: f64, t536: f64, t1747: f64, t6302: f64, t1798: f64, t6488: f64, t543: f64, t1887: f64, t337: f64, t1782: f64, t1672: f64, t1778: f64) -> (f64, f64, f64, f64, f64) {
    let t7332 = t536 * t1468;
    let t7333 = t7332 * t1747;
    let t7335 = 4.4281498357666145_f64 * t7333 * t6302;
    let t7337 = 1.4760499452555382_f64 * t1798 * t6488;
    let t7339 = t543 * t543;
    let t7340 = 1.0_f64 / t7339;
    let t7345 = t1887 * t337;
    let t7346 = t7345 * t1782;
    let t7353 = t1778 * t1672;
    (t7335, t7337, t7340, t7346, t7353)
}

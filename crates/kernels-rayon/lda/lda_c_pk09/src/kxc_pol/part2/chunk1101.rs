//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1101/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1101(t1887: f64, t2897: f64, t1792: f64, t2888: f64, t12114: f64, t12133: f64, t1888: f64, t534: f64, t6849: f64, t6853: f64, t1896: f64, t452: f64) -> f64 {
    let t12135 = t2897 * t1887;
    let t12140 = t2888 * t1792;
    let t12145 = t12133 * t534 - t12135 * t1792 / 2.0_f64 - t6849 * t2888 / 2.0_f64 + 3.0_f64 / 4.0_f64 * t6853 * t12140 - t1888 * t12114 / 2.0_f64;
    let t12146 = t12145 * t1896;
    let t12147 = t12146 * t452;
    t12147
}

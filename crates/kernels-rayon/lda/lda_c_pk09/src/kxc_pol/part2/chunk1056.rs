//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1056/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1056(t11509: f64, t2042: f64, t2795: f64, t7286: f64, t11092: f64, t7296: f64, t462: f64, t476: f64, t1672: f64, t2856: f64, t11218: f64, t6511: f64, t6524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11510 = t11509 * t2042;
    let t11512 = t2795 * t7286;
    let t11515 = t7296 * t11092;
    let t11517 = t462 * t476;
    let t11520 = t2856 * t1672;
    let t11529 = t6524 * t6511 * t11218;
    (t11510, t11512, t11515, t11517, t11520, t11529)
}

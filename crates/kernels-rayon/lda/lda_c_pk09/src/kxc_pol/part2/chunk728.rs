//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 728/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk728(t1798: f64, t6292: f64, t489: f64, t6287: f64, t497: f64, t1831: f64, t1800: f64, t1827: f64, t501: f64, t1971: f64, t309: f64, t1876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7488 = 2.2140749178833072_f64 * t1798 * t6292;
    let t7489 = t489 * t6287;
    let t7494 = t497 * t6287;
    let t7500 = t1831 * t6287;
    let t7501 = t7500 * t1800;
    let t7503 = t1827 * t6287;
    let t7504 = t7503 * t1800;
    let t7506 = t501 * t6287;
    let t7513 = t1971 * t309;
    let t7516 = t1876 * t6287;
    (t7488, t7489, t7494, t7501, t7504, t7506, t7513, t7516)
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1048/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1048(t2777: f64, t6945: f64, t452: f64, t1941: f64, t309: f64, t454: f64, t1947: f64, t2919: f64, t2042: f64, t1240: f64, t2923: f64, t1905: f64) -> (f64, f64, f64, f64) {
    let t11400 = t2777 * t6945;
    let t11401 = t11400 * t452;
    let t11403 = t309 * t454 * t1941;
    let t11406 = t2919 * t1947;
    let t11407 = t11406 * t2042;
    let t11411 = t2923 * t1240;
    let t11412 = t1905 * t11411;
    (t11401, t11403, t11407, t11412)
}

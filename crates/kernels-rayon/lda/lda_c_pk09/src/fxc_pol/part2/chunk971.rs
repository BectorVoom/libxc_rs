//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 971/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk971(t10314: f64, t10330: f64, t323: f64, t306: f64, t1215: f64, t2551: f64, t130: f64, t2550: f64, t93: f64, t1593: f64, t2487: f64, t10104: f64, t327: f64) -> (f64, f64, f64, f64, f64) {
    let t10331 = t10314 + t10330;
    let t10332 = t323 * t10331;
    let t10333 = t10332 * t306;
    let t10341 = t2551 * t1215;
    let t10345 = t130 * t2550;
    let t10346 = t93 * t10345;
    let t10349 = t1593 * t2487;
    let t10352 = t327 * t10104;
    (t10333, t10341, t10346, t10349, t10352)
}

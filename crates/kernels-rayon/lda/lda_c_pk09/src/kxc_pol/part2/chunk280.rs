//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 280/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk280(t1255: f64, t48: f64, t623: f64, t285: f64, t284: f64) -> (f64, f64, f64) {
    let t1256 = 1.0694947305092268_f64 * t1255;
    let t1257 = t48 * t623;
    let t1258 = t285 * t1257;
    let t1259 = t284 * t1258;
    (t1256, t1257, t1259)
}

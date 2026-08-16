//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 929/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk929(t359: f64, t9739: f64, t355: f64, t347: f64, t1222: f64, t2512: f64) -> (f64, f64, f64, f64) {
    let t9827 = t359 * t9739;
    let t9830 = t355 * t9739;
    let t9833 = t347 * t9739;
    let t9836 = t1222 * t2512;
    (t9827, t9830, t9833, t9836)
}

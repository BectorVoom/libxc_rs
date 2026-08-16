//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 128/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk128(t280: f64, t395: f64, t287: f64, t294: f64, t305: f64) -> (f64, f64, f64, f64) {
    let t396 = t395 * t280;
    let t401 = 3.125_f64 * t287 + 1.2466946262544771_f64 * t294 + 0.146484375_f64;
    let t402 = f64::ln(t401);
    let t403 = t402 * t305;
    (t396, t401, t402, t403)
}

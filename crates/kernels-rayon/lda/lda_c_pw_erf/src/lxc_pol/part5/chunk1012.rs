//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1012/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1012(t2416: f64, t925: f64, t2520: f64, t933: f64, t2523: f64, t2517: f64, t325: f64, t6666: f64, t331: f64, t6802: f64, t2420: f64, t2412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16345 = t925 * t2416;
    let t16365 = t933 * t2520;
    let t16370 = t933 * t2523;
    let t16372 = t933 * t2517;
    let t16374 = t325 * t6666;
    let t16382 = t331 * t6802;
    let t16397 = t925 * t2420;
    let t16399 = t925 * t2412;
    (t16345, t16365, t16370, t16372, t16374, t16382, t16397, t16399)
}

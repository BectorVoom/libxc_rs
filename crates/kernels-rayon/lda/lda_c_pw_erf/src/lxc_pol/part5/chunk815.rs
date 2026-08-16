//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 815/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk815(t199: f64, t7466: f64, t2400: f64, t820: f64, t184: f64, t221: f64, t4465: f64, t1460: f64, t7354: f64, t522: f64, t519: f64, t1486: f64, t7365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7468 = 4.0_f64 / 5.0_f64 * t7466 * t199;
    let t7469 = t2400 * t820;
    let t7470 = t7469 * t184;
    let t7472 = 4.0_f64 / 5.0_f64 * t7470 * t221;
    let t7473 = 4.0_f64 / 45.0_f64 * t4465;
    let t7474 = t1460 * t7354;
    let t7475 = t522 * t7474;
    let t7477 = 8.0_f64 / 15.0_f64 * t519 * t7475;
    let t7478 = t1486 * t7365;
    (t7468, t7469, t7470, t7472, t7473, t7474, t7475, t7477, t7478)
}

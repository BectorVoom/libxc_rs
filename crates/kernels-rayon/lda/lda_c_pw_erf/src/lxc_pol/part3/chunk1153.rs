//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1153/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1153(t2163: f64, t3742: f64, t1466: f64, t3669: f64, t571: f64, t833: f64, t9237: f64, t1318: f64, t2157: f64, t9432: f64, t3732: f64, t4738: f64) -> (f64, f64, f64, f64) {
    let t13500 = 8.0_f64 / 5.0_f64 * t3742 * t2163;
    let t13505 = 16.0_f64 / 5.0_f64 * t571 * t1466 * t9237 * t833 * t3669;
    let t13507 = t1318 * t9432 * t2157;
    let t13508 = 8.0_f64 / 45.0_f64 * t13507;
    let t13510 = 8.0_f64 / 5.0_f64 * t4738 * t3732;
    (t13500, t13505, t13508, t13510)
}

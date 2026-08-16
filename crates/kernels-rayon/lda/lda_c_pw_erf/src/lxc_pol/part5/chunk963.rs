//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 963/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk963(t11898: f64, t2130: f64, t493: f64, t10162: f64, t1325: f64, t2182: f64, t108: f64, t2113: f64, t267: f64, t2010: f64, t571: f64, t9313: f64) -> (f64, f64, f64, f64) {
    let t12987 = t493 * t11898 * t2130;
    let t12998 = t1325 * t10162 * t2182;
    let t12999 = 8.0_f64 / 45.0_f64 * t12998;
    let t13035 = t2113 * t108 * t267;
    let t13048 = t571 * t9313 * t2010;
    (t12987, t12999, t13035, t13048)
}

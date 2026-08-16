//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1018/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1018(t12121: f64, t2471: f64, t12064: f64, t6734: f64, t6737: f64, t581: f64, t6843: f64, t1294: f64, t2425: f64, t4568: f64, t6209: f64, t2127: f64, t6580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16867 = t12121 * t2471;
    let t16874 = t12064 * t6734;
    let t16876 = t12064 * t6737;
    let t16907 = t581 * t6843;
    let t16912 = t2425 * t1294;
    let t16918 = t6209 * t4568;
    let t16922 = t6580 * t2127;
    (t16867, t16874, t16876, t16907, t16912, t16918, t16922)
}

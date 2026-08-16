//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 424/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk424(t1995: f64, t493: f64, t1447: f64, t835: f64, t1423: f64, t806: f64, t224: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1997 = t493 * t1995 / 15.0_f64;
    let t1998 = t1447 * t835;
    let t1999 = 2.0_f64 / 135.0_f64 * t1998;
    let t2000 = t1423 * t806;
    let t2001 = 2.0_f64 / 135.0_f64 * t2000;
    let t2002 = t801 * t224;
    (t1997, t1998, t1999, t2000, t2001, t2002)
}

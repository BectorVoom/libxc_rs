//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 996/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk996(t13726: f64, t806: f64, t2007: f64, t5220: f64, t2012: f64, t5210: f64, t801: f64, t2481: f64, t3220: f64, t1423: f64, t6241: f64, t6245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17991 = t13726 * t806;
    let t17993 = t5220 * t2007;
    let t17996 = t801 * t5210 * t2012;
    let t18002 = t3220 * t2481;
    let t18004 = t1423 * t6241;
    let t18006 = t1423 * t6245;
    (t17991, t17993, t17996, t18002, t18004, t18006)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1028/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1028(t10011: f64, t6771: f64, t10030: f64, t6756: f64, t2337: f64, t352: f64, t3863: f64, t571: f64, t6396: f64, t13444: f64, t6400: f64, t13080: f64, t1318: f64, t6482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17657 = t10011 * t6771;
    let t17664 = t10030 * t6756;
    let t17673 = t2337 * t352;
    let t17684 = t571 * t3863 * t6396;
    let t17687 = t571 * t13444 * t6400;
    let t17690 = t1318 * t13080 * t6482;
    (t17657, t17664, t17673, t17684, t17687, t17690)
}

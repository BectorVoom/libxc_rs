//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1009/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1009(t108: f64, t209: f64, t12781: f64, t1325: f64, t6432: f64, t504: f64, t6566: f64, t10463: f64, t2392: f64, t12695: f64, t6229: f64, t3863: f64, t571: f64, t6286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16144 = t209 * t108;
    let t16159 = t1325 * t12781 * t6432;
    let t16209 = t6566 * t504;
    let t16221 = t1325 * t10463 * t2392;
    let t16224 = t1325 * t12695 * t6229;
    let t16232 = t571 * t3863 * t6286;
    (t16144, t16159, t16209, t16221, t16224, t16232)
}

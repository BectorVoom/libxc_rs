//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 389/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk389(t1738: f64, t286: f64, t116: f64, t299: f64, t732: f64, t310: f64, t311: f64) -> (f64, f64, f64) {
    let t1740 = 0.05321881782335382_f64 * t1738 * t286;
    let t1746 = t732 * t299 * t116;
    let t1750 = 1.0_f64 / t311 / t310;
    (t1740, t1746, t1750)
}

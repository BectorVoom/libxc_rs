//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 544/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk544(t1193: f64, t1354: f64, t2822: f64, t1186: f64, t1343: f64, t421: f64, t398: f64, t740: f64, t1183: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t2825 = 0.0034679929861433484_f64 * t2822 * t1193 * t1354;
    let t2831 = t1343 * t1186 * t421;
    let t2833 = t740 * t398;
    let t2835 = t2833 * t1193 * t1354;
    let t2837 = t1183 * t27;
    (t2825, t2831, t2833, t2835, t2837)
}

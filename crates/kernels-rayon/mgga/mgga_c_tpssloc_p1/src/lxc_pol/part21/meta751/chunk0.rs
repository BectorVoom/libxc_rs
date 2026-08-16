//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2623/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2623(t1831: f64, t40292: f64, t12345: f64, t5314: f64, t12211: f64, t16296: f64, t40018: f64, t5223: f64, t16379: f64, t40021: f64, t12282: f64, t5234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53917 = t40292 * t1831;
    let t53919 = t12345 * t5314;
    let t53921 = t12211 * t16296;
    let t53927 = t40018 * t5223;
    let t53929 = t40021 * t16379;
    let t53945 = t5234 * t12282;
    (t53917, t53919, t53921, t53927, t53929, t53945)
}

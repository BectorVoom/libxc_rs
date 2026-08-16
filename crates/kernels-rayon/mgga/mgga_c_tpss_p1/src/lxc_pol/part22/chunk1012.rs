//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1012/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1012(t10952: f64, t912: f64, t2629: f64, t3909: f64, t1485: f64, t9133: f64, t3762: f64, t845: f64, t867: f64, t2526: f64, t3765: f64, t1411: f64, t2530: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10954 = 0.11696447245269292414e1_f64 * t912 * t10952;
    let t10956 = 0.34631718211362927518e2_f64 * t2629 * t3909;
    let t10957 = t1485 * t9133;
    let t10961 = t3762 * t845;
    let t10963 = 2.0_f64 * t10961 * t867;
    let t10965 = 1.0_f64 * t3765 * t2526;
    let t10966 = t1411 * t2530;
    (t10954, t10956, t10957, t10963, t10965, t10966)
}

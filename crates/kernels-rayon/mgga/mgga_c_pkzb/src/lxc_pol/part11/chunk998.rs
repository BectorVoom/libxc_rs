//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 998/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk998(t10964: f64, t730: f64, t2860: f64, t3622: f64, t3626: f64, t1116: f64, t9242: f64, t3618: f64, t10757: f64, t10891: f64, t10951: f64, t10954: f64, t10958: f64, t10962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10966 = 0.10389515463408878255e3_f64 * t730 * t10964;
    let t10968 = 0.17544670867903938621e1_f64 * t2860 * t3622;
    let t10970 = 0.51947577317044391276e2_f64 * t2860 * t3626;
    let t10972 = 0.17544670867903938621e1_f64 * t9242 * t1116;
    let t10974 = 0.35089341735807877242e1_f64 * t2860 * t3618;
    let t10975 = t10757 - t10951 - t10954 - t10958 - t10962 + t10966 - t10968 - t10970 - t10972 + t10974 - t10891;
    (t10966, t10968, t10970, t10972, t10974, t10975)
}

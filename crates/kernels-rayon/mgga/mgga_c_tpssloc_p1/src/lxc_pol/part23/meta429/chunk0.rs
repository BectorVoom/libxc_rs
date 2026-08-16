//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1265/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1265(t21788: f64, t699: f64, t21791: f64, t21938: f64, t3403: f64, t21809: f64, t3315: f64, t21886: f64, t3359: f64, t1147: f64, t21826: f64, t1128: f64, t21975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71472 = t699 * t21788;
    let t71474 = t699 * t21791;
    let t71672 = t21938 * t3403;
    let t71701 = t21809 * t3315;
    let t71729 = t21886 * t3359;
    let t71860 = t21826 * t1147;
    let t71863 = t21975 * t1128;
    (t71472, t71474, t71672, t71701, t71729, t71860, t71863)
}

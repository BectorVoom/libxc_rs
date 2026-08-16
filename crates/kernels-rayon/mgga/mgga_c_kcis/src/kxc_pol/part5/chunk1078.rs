//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1078/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1078(t1094: f64, t6480: f64, t1122: f64, t1092: f64, t6708: f64, t1134: f64, t6487: f64, t9532: f64, t13192: f64, t4807: f64, t2825: f64, t6629: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18458 = t6480 * t1094;
    let t18459 = t18458 * sigma0;
    let t18460 = t18459 * t1122;
    let t18461 = t1092 * t18460;
    let t18463 = t6708 * sigma0;
    let t18464 = t18463 * t1134;
    let t18465 = t1092 * t18464;
    let t18467 = t9532 * t6487;
    let t18468 = t1092 * t18467;
    let t18471 = t13192 * t4807;
    let t18473 = t2825 * t6629;
    (t18458, t18461, t18463, t18465, t18468, t18471, t18473)
}

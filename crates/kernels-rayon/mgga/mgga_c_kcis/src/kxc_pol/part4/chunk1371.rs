//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1371/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1371(t17562: f64, t17602: f64, t17648: f64, t17688: f64, t552: f64, t573: f64, t12542: f64, t2055: f64, t17453: f64, t5909: f64, t4260: f64, t2043: f64, t4245: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t17690 = t17562 + t17602 + t17648 + t17688;
    let t17691 = t17690 * t552;
    let t17692 = t17691 * sigma2;
    let t17693 = t17692 * t573;
    let t17695 = t12542 * t2055;
    let t17697 = t5909 * t17453;
    let t17698 = t4260 * t17697;
    let t17700 = t4245 * t2043;
    (t17693, t17695, t17698, t17700)
}

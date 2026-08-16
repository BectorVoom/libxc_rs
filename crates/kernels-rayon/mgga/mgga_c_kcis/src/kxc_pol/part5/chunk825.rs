//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 825/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk825(t1662: f64, t1727: f64, t3274: f64, t1103: f64, t3279: f64, t6272: f64, t1104: f64, t6276: f64, t3288: f64, t6320: f64, t345: f64, t4606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6578 = t3274 * t1662 * t1727;
    let t6582 = t1103 * t3279 * t6272;
    let t6586 = t1103 * t1104 * t6276;
    let t6589 = t3288 * t6320;
    let t6590 = t345 * t6589;
    let t6593 = t4606 * t1727;
    (t6578, t6582, t6586, t6589, t6590, t6593)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 927/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk927(t158: f64, t7011: f64, t7026: f64, t7044: f64, t7052: f64, t133: f64, t594: f64, t1020: f64, t1773: f64, t1634: f64, t2575: f64, t614: f64) -> (f64, f64, f64, f64, f64) {
    let t7055 = (t7011 + t7026 + t7044 + t7052) * t158;
    let t7065 = t594 * t133;
    let t7070 = t1773 * t1020;
    let t7071 = t7070 * t1634;
    let t7074 = t614 * t2575;
    (t7055, t7065, t7070, t7071, t7074)
}

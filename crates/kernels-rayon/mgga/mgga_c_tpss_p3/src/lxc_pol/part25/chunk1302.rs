//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1302/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1302(t1206: f64, t5458: f64, t6242: f64, t7309: f64, t14001: f64, t196: f64, t197: f64, t1268: f64, t21011: f64, t1338: f64, t3490: f64, t1321: f64, t3537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68958 = t5458 * t1206;
    let t68967 = t6242 * t7309;
    let t68975 = t14001 * t196 * t197;
    let t68989 = t21011 * t1268;
    let t69023 = t3490 * t1338;
    let t69026 = t1321 * t3537;
    (t68958, t68967, t68975, t68989, t69023, t69026)
}

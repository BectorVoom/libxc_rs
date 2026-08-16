//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 879/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk879(t2051: f64, t2066: f64, t2055: f64, t6002: f64, t2054: f64, t2061: f64, t1546: f64, t4293: f64, t6917: f64, t4292: f64, t2039: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7292 = t2051 * t2066;
    let t7294 = t6002 * t2055;
    let t7296 = t2061 * t2054;
    let t7297 = t1546 * t7296;
    let t7299 = t4293 * t6917;
    let t7300 = t4292 * t7299;
    let t7302 = t6016 * t2039;
    (t7292, t7294, t7296, t7297, t7299, t7300, t7302)
}

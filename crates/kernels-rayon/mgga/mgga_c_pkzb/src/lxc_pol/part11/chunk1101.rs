//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1101/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1101(t1667: f64, t6801: f64, t2609: f64, t5336: f64, t16940: f64, t1542: f64, t2607: f64, t2663: f64, t5296: f64, t1025: f64, t16378: f64, t16421: f64, t183: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20372 = t6801 * t1667;
    let t20373 = 0.73245789224026180216e-3_f64 * t20372;
    let t20374 = t2609 * t5336;
    let t20377 = 192.0_f64 * t16940;
    let t20378 = t1542 * t2607;
    let t20407 = t5296 * t2663;
    let t20409 = t16378 * t1025;
    let t20542 = t16421 * t183;
    (t20373, t20374, t20377, t20378, t20407, t20409, t20542)
}

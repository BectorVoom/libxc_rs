//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1048/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1048(t16226: f64, t534: f64, t541: f64, t555: f64, t137: f64, t1835: f64, t139: f64, t2177: f64, t1516: f64, t490: f64, t4994: f64, t1542: f64, t1626: f64) -> (f64, f64, f64, f64, f64) {
    let t16230 = 0.5848223622634646207e0_f64 * t555 * t534 * t16226 * t541;
    let t16232 = 1.0_f64 / t137 / t1835;
    let t16250 = 1.0_f64 / t139 / t2177;
    let t16273 = 8.0_f64 * t1516 * t4994 * t490;
    let t16275 = 120.0_f64 * t1542 * t1626;
    (t16230, t16232, t16250, t16273, t16275)
}

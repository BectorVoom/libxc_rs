//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1978/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1978(t1336: f64, t22759: f64, t835: f64, t22760: f64, t3777: f64, t12248: f64, t6604: f64, t22716: f64, t6983: f64, t22723: f64, t268: f64, t534: f64) -> (f64, f64, f64, f64, f64) {
    let t80997 = t1336 * t22759 * t835;
    let t81000 = t3777 * t22760;
    let t81027 = t6604 * t12248;
    let t81039 = t22716 * t6983;
    let t81046 = t22723 * t534 * t268;
    (t80997, t81000, t81027, t81039, t81046)
}

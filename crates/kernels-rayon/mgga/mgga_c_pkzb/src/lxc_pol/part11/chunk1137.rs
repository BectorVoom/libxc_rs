//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1137/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1137(t2096: f64, t9616: f64, t5713: f64, t9605: f64, t2038: f64, t3656: f64, t5939: f64, t179: f64, t299: f64, t3515: f64, t5672: f64, t771: f64, t9628: f64) -> (f64, f64, f64, f64, f64) {
    let t25530 = t2096 * t9616;
    let t25553 = t5713 * t9605;
    let t25556 = t2038 * t5939 * t3656;
    let t25572 = t299 * t179 * t5672 * t3515;
    let t25576 = t771 * t9628;
    (t25530, t25553, t25556, t25572, t25576)
}

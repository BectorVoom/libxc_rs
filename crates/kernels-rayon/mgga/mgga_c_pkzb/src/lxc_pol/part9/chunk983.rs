//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 983/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk983(t7618: f64, t7686: f64, t7749: f64, t7802: f64, t158: f64, t1143: f64, t2119: f64, t6000: f64, t2118: f64, t2989: f64, t799: f64, t2145: f64, t2964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7804 = t7618 + t7686 + t7749 + t7802;
    let t7805 = t7804 * t158;
    let t7821 = t6000 * t1143 * t2119;
    let t7824 = t2118 * t2989;
    let t7825 = t7824 * t799;
    let t7828 = t2964 * t2145;
    (t7804, t7805, t7821, t7824, t7825, t7828)
}

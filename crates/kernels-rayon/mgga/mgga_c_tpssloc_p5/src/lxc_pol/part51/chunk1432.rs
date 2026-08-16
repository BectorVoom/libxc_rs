//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1432/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1432(t1985: f64, t8621: f64, t90739: f64, t115545: f64, t1992: f64, t26355: f64, t22633: f64, t22635: f64, t31549: f64, t5187: f64, t33272: f64, t81228: f64, t81326: f64) -> (f64, f64, f64, f64) {
    let t122260 = t1985 * t90739 * t8621;
    let t122270 = t1992 * t115545 * t26355;
    let t122278 = t22633 * t22635 * t31549 * t5187;
    let t122281 = t81228 * t81326 * t33272;
    (t122260, t122270, t122278, t122281)
}

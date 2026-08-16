//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1218/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1218(t1799: f64, t31549: f64, t22635: f64, t22633: f64, t31618: f64, t6637: f64, t6888: f64, t27074: f64, t550: f64, t6976: f64, t1992: f64, t1998: f64, t7918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33272 = t31549 * t1799;
    let t33273 = t22635 * t33272;
    let t33274 = t22633 * t33273;
    let t33276 = t31618 * t1799;
    let t33277 = t6637 * t33276;
    let t33278 = t6888 * t33277;
    let t33280 = t27074 * t550;
    let t33281 = t6976 * t33280;
    let t33282 = t1992 * t33281;
    let t33284 = t1998 * t7918;
    (t33272, t33273, t33274, t33276, t33277, t33278, t33280, t33281, t33282, t33284)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 383/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk383(t1511: f64, t557: f64, t470: f64, t71: f64, t57: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1512 = t1511 * t557;
    let t1513 = 0.11696447245269292414e1_f64 * t1512;
    let t1514 = t470 * t71;
    let t1515 = 1.0_f64 / t1514;
    let t1516 = t57 * t1515;
    let t1517 = t490 * t490;
    (t1512, t1513, t1514, t1515, t1516, t1517)
}

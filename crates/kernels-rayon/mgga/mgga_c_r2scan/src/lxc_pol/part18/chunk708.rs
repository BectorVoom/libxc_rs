//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 708/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk708(t1399: f64, t1660: f64, t1665: f64, t2005: f64, t206: f64, t1923: f64, t2008: f64, t1966: f64, t689: f64, t1937: f64, t681: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5612 = 0.14246666666666666666e0_f64 * t1399 * t1660;
    let t5614 = 0.11455730062901982479e1_f64 * t1399 * t1665;
    let t5627 = t2005 * t206;
    let t5628 = t2008 * t1923;
    let t5629 = t5627 * t5628;
    let t5632 = t689 * t1966;
    let t5633 = t1937 * t5632;
    let t5636 = t686 * t681;
    (t5612, t5614, t5629, t5632, t5633, t5636)
}

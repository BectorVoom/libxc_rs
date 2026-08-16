//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2071/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071(t7557: f64, t82632: f64, t25836: f64, t3216: f64, t11094: f64, t7627: f64, t28: f64, t40772: f64, t1649: f64, t2752: f64, t26012: f64, t6505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89672 = t82632 * t7557;
    let t89698 = t25836 * t3216;
    let t89702 = t7627 * t11094;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90087 = t6505 * t26012;
    (t89672, t89698, t89702, t89953, t89992, t90087)
}

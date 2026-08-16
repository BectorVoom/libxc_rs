//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1224/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1224(t1775: f64, t987: f64, t1163: f64, t1165: f64, t1552: f64, t1759: f64, t879: f64, t322: f64, t6263: f64, t1532: f64, t3456: f64, t3372: f64, t6405: f64) -> (f64, f64, f64, f64, f64) {
    let t22383 = t987 * t1775;
    let t22388 = t1163 * t1165 * t1552 * t1759 * t879;
    let t22394 = t6263 * t322;
    let t22397 = t3456 * t1165 * t1532 * t22394;
    let t22399 = t3372 * t6405;
    (t22383, t22388, t22394, t22397, t22399)
}

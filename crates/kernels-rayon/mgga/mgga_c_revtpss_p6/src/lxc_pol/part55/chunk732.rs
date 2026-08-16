//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 732/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk732(t1294: f64, t2142: f64, t7652: f64, t3140: f64, t487: f64, t1276: f64, t2148: f64, t1243: f64, t1248: f64, t1287: f64, t2150: f64, t473: f64, t7627: f64) -> (f64, f64, f64, f64, f64) {
    let t7653 = t2142 * t1294;
    let t7654 = t7652 * t7653;
    let t7657 = t487 * t3140;
    let t7658 = t7657 * t1276;
    let t7659 = t2148 * t7658;
    let t7660 = t1243 * t2142;
    let t7662 = t7660 * t1248 * t1287;
    let t7666 = t2150 * t473 * t7627;
    (t7654, t7659, t7660, t7662, t7666)
}

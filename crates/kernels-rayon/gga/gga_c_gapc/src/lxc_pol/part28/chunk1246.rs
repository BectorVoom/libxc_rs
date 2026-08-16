//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1246/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1246(t11488: f64, t1688: f64, t21157: f64, t1743: f64, t33958: f64, t34711: f64, t1036: f64, t11316: f64, t15341: f64, t1030: f64, t12768: f64, t1749: f64) -> (f64, f64, f64, f64) {
    let t34779 = t11488 * t1688 * t21157;
    let t34782 = t1743 * t33958 * t34711;
    let t34785 = t11316 * t1036 * t15341;
    let t34788 = t1030 * t12768 * t1749;
    (t34779, t34782, t34785, t34788)
}

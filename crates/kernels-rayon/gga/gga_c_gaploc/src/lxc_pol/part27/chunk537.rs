//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 537/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk537(t1064: f64, t2779: f64, t2778: f64, t550: f64, t1365: f64, t599: f64, t986: f64) -> (f64, f64, f64, f64) {
    let t2780 = t1064 * t2779;
    let t2783 = t550 * t2778;
    let t2784 = t1365 * t2783;
    let t2787 = t599 * t986;
    (t2780, t2783, t2784, t2787)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 699/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk699(t1360: f64, t2317: f64, t6525: f64, t1365: f64, t4325: f64, t550: f64, t6417: f64, t158: f64, t2293: f64) -> (f64, f64, f64, f64, f64) {
    let t6526 = t1360 * t2317;
    let t6527 = t6525 * t6526;
    let t6533 = t1365 * t4325;
    let t6534 = t6525 * t6533;
    let t6536 = t550 * t6417;
    let t6537 = t1365 * t6536;
    let t6540 = t158 * t2293;
    (t6527, t6534, t6536, t6537, t6540)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1387/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1387(t10318: f64, t1646: f64, t4398: f64, t26609: f64, t6628: f64, t10170: f64, t107: f64, t1415: f64, t1417: f64, t30805: f64, t30808: f64, t30810: f64, t30813: f64, t30821: f64, t30824: f64, t30828: f64, t30833: f64, t34592: f64, t34595: f64, t34603: f64, t34607: f64, t34609: f64) -> f64 {
    let t34612 = 0.71500979903700853338e0_f64 * t4398 * t10318 * t1646;
    let t34614 = 0.21450293971110256002e1_f64 * t26609 * t6628;
    let t34615 = -t34592 + t34595 + t30805 - t30808 + 0.79445533226334281486e-1_f64 * t1415 * t10170 * t107 * t1417 + t30810 + t30813 + t30821 - t30824 - t30828 - t30833 - t34603 - t34607 - t34609 - t34612 - t34614;
    t34615
}

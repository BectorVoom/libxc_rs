//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 728/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk728(t533: f64, t6995: f64, t1390: f64, t1983: f64, t1388: f64, t3701: f64) -> (f64, f64, f64, f64) {
    let t6996 = t533 * t6995;
    let t6997 = t6996 * t1390;
    let t6998 = t1983 * t6997;
    let t6999 = t3701 * t1388;
    (t6996, t6997, t6998, t6999)
}

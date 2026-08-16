//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1841/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1841(t1888: f64, t23270: f64, t2719: f64, t46488: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t22986: f64, t23012: f64, t7489: f64, t13460: f64, t1880: f64, t6553: f64, t6571: f64) -> (f64, f64, f64, f64, f64) {
    let t86961 = t1888 * t23270 * t46488 * t2719;
    let t86967 = t6579 * t25046;
    let t86969 = t2717 * t1484;
    let t86972 = t22986 * t23270 * t86969 * t2719;
    let t86991 = t23012 * t7489;
    let t86997 = t1880 * t6553 * t6571 * t13460;
    (t86961, t86967, t86972, t86991, t86997)
}

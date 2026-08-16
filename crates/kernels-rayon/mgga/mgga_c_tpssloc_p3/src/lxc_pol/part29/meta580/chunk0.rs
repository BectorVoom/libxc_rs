//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1998/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1998(t22705: f64, t22852: f64, t550: f64, t80786: f64, t22823: f64, t281: f64, t22855: f64, t3862: f64, t6940: f64, t1358: f64, t22836: f64, t22690: f64, t3787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80789 = t22852 * t22705 * t80786 * t550;
    let t80791 = t22823 * t281;
    let t80792 = t80791 * t22855;
    let t80794 = t6940 * t3862;
    let t80796 = t22836 * t1358;
    let t80798 = t22690 * t3787;
    (t80789, t80791, t80792, t80794, t80796, t80798)
}

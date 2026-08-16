//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2142/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2142(t87618: f64, t1902: f64, t4233: f64, t1888: f64, t232: f64, t47528: f64, t6646: f64, t13398: f64, t82018: f64, t13404: f64, t22996: f64, t7521: f64, t81632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87619 = 0.16449340668482264365e-1_f64 * t87618;
    let t87620 = t1902 * t4233;
    let t87627 = t1888 * t6646 * t47528 * t232;
    let t87630 = t1888 * t82018 * t13398;
    let t87633 = t1888 * t22996 * t13404;
    let t87635 = t81632 * t7521;
    (t87619, t87620, t87627, t87630, t87633, t87635)
}

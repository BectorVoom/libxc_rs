//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1258/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1258(t1458: f64, t7263: f64, t1874: f64, t2113: f64, t4072: f64, t33690: f64, t6525: f64, t31832: f64, t7756: f64, t25992: f64, t8690: f64, t24991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122917 = t7263 * t1458;
    let t122918 = t122917 * t1874;
    let t122920 = t2113 * t4072;
    let t122921 = t122920 * t1874;
    let t122923 = t33690 * t6525;
    let t122925 = t31832 * t7756;
    let t123027 = t8690 * t25992;
    let t123028 = t8690 * t24991;
    (t122917, t122918, t122920, t122921, t122923, t122925, t123027, t123028)
}

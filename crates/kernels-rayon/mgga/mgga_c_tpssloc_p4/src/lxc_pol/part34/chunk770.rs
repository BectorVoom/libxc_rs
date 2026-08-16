//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 770/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk770(t1512: f64, t9671: f64, t1509: f64, t2632: f64, t1500: f64, t2693: f64, t2642: f64, t4166: f64, t2638: f64, t2629: f64, t2696: f64, t1516: f64, t9601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13182 = t9671 * t1512;
    let t13228 = t1509 * t2632;
    let t13234 = t1500 * t2693;
    let t13251 = t4166 * t2642;
    let t13278 = t4166 * t2638;
    let t13283 = t4166 * t2629;
    let t13360 = t4166 * t2696;
    let t13368 = t9601 * t1516;
    (t13182, t13228, t13234, t13251, t13278, t13283, t13360, t13368)
}

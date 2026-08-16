//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 857/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk857(t13123: f64, t2375: f64, t1512: f64, t9671: f64, t2644: f64, t820: f64, t1509: f64, t2632: f64, t1500: f64, t2693: f64, t2642: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13124 = t13123 * t2375;
    let t13182 = t9671 * t1512;
    let t13222 = t2644 * t820;
    let t13228 = t1509 * t2632;
    let t13234 = t1500 * t2693;
    let t13251 = t4166 * t2642;
    (t13124, t13182, t13222, t13228, t13234, t13251)
}

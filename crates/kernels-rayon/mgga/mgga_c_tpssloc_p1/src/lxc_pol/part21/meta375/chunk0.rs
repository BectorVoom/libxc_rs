//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1824/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1824(t13748: f64, t973: f64, t1611: f64, t3088: f64, t1036: f64, t4617: f64, t1023: f64, t4347: f64, t3071: f64, t10422: f64, t4574: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13750 = t973 * t13748 / 216.0_f64;
    let t13751 = t1611 * t3088;
    let t13758 = t4617 * t1036 / 2304.0_f64;
    let t13761 = t4347 * t1023;
    let t13762 = t3071 * t13761;
    let t13765 = t10422 * t4574;
    let t13767 = t3070 * t13765 / 3456.0_f64;
    (t13750, t13751, t13758, t13761, t13762, t13765, t13767)
}

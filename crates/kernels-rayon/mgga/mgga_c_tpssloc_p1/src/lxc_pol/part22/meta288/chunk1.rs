//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1443/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1443(t1484: f64, t828: f64, t1516: f64, t9993: f64, t2696: f64, t4166: f64) -> (f64, f64, f64) {
    let t13351 = t1484 * t828;
    let t13359 = 7.0_f64 / 576.0_f64 * t9993 * t1516;
    let t13360 = t4166 * t2696;
    (t13351, t13359, t13360)
}

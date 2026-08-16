//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1249/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1249(t10254: f64, t21510: f64, t21472: f64, t2970: f64, t973: f64, t13822: f64, t21452: f64, t21468: f64, t42972: f64, t21682: f64, t225: f64, t1009: f64, t21480: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69746 = t10254 * t21510;
    let t69796 = t973 * t2970 * t21472;
    let t69801 = t973 * t13822 * t21452;
    let t69806 = t973 * t42972 * t21468;
    let t69871 = t21682 * t225;
    let t69923 = t21480 * t1009;
    (t69746, t69796, t69801, t69806, t69871, t69923)
}

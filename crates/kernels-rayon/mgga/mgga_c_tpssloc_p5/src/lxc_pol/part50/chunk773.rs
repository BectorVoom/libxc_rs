//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 773/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk773(t1539: f64, t6785: f64, t6784: f64, t1599: f64, t1949: f64, t1629: f64, t6800: f64, t6799: f64, t1625: f64, t1948: f64, t345: f64, t1615: f64, t1945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7603 = t6785 * t1539;
    let t7604 = t6784 * t7603;
    let t7607 = t1599 * t1949;
    let t7610 = t1629 * t6800;
    let t7611 = t6799 * t7610;
    let t7614 = t1948 * t1625;
    let t7615 = t345 * t7614;
    let t7619 = t1945 * t1615;
    (t7603, t7604, t7607, t7610, t7611, t7614, t7615, t7619)
}

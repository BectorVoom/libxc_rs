//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 632/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk632(t1653: f64, t7286: f64, t7285: f64, t1716: f64, t2123: f64, t1751: f64, t225: f64, t497: f64, t462: f64, t1760: f64, t7301: f64, t7300: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8002 = t7286 * t1653;
    let t8003 = t7285 * t8002;
    let t8006 = t1716 * t2123;
    let t8009 = t1751 * t225;
    let t8010 = t8009 * t497;
    let t8011 = t462 * t8010;
    let t8014 = t7301 * t1760;
    let t8015 = t7300 * t8014;
    (t8002, t8003, t8006, t8010, t8011, t8014, t8015)
}

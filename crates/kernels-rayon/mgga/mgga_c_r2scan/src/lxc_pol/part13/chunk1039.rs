//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1039/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1039(t545: f64, t7600: f64, t146: f64, t6091: f64, t978: f64, t113: f64, t24877: f64, t2573: f64, t481: f64, t1550: f64, t938: f64, t2145: f64, t2832: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t26307 = t24877 * t113;
    let t26314 = t2573 * t481;
    let t26997 = t938 * t1550 * t113;
    let t27067 = t146 * t2145 * t2832;
    (t26278, t26282, t26307, t26314, t26997, t27067)
}

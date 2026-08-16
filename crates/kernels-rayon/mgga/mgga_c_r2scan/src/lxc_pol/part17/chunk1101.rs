//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1101/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1101(t2553: f64, t37764: f64, t10894: f64, t2630: f64, t10844: f64, t11760: f64, t2201: f64, t2214: f64, t3293: f64, t528: f64, t132: f64, t1567: f64) -> (f64, f64, f64, f64, f64) {
    let t39579 = t37764 * t2553;
    let t39601 = t10894 * t2630;
    let t39607 = t2201 * t11760 * t10844;
    let t39613 = t3293 * t2214 * t528;
    let t39614 = t132 * t1567;
    (t39579, t39601, t39607, t39613, t39614)
}

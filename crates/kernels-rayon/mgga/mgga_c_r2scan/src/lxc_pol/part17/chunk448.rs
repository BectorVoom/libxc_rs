//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 448/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk448(t1632: f64, t560: f64, t551: f64, t549: f64, t146: f64, t1541: f64) -> (f64, f64, f64, f64) {
    let t2190 = t1632 * t560;
    let t2191 = t551 * t2190;
    let t2192 = t549 * t2191;
    let t2195 = t146 * t1541;
    (t2190, t2191, t2192, t2195)
}

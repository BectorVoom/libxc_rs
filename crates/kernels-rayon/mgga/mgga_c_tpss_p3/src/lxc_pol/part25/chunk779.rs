//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 779/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk779(t124: f64, t5371: f64, t762: f64, t5366: f64, t1639: f64) -> (f64, f64, f64, f64, f64) {
    let t5372 = t124 * t5371;
    let t5373 = t762 * t5372;
    let t5376 = t124 * t5366;
    let t5377 = t762 * t5376;
    let t5380 = t1639 * t1639;
    (t5372, t5373, t5376, t5377, t5380)
}

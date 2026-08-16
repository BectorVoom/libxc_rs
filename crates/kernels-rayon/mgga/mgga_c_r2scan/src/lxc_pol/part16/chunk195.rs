//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 195/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk195(t167: f64, t585: f64, t377: f64, t5: f64, t390: f64) -> (f64, f64, f64, f64, f64) {
    let t586 = t585 * t167;
    let t587 = t5 * t377;
    let t588 = t586 * t587;
    let t590 = 0.1046175e-1_f64 * t390;
    let t591 = -0.14816666666666666667e-1_f64 * t588 - t590;
    (t586, t587, t588, t590, t591)
}

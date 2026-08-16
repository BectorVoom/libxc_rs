//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 825/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk825(t108: f64, t8633: f64, t8639: f64, t8645: f64, t8659: f64, t915: f64, t95: f64, t2892: f64, t5052: f64, t481: f64, t2505: f64, t2526: f64) -> (f64, f64, f64, f64) {
    let t8662 = (t8633 + t8639 + t8645 + t8659) * t108;
    let t8668 = t915 * t95;
    let t8675 = t5052 * t2892;
    let t8676 = t8675 * t481;
    let t8679 = t2505 * t2526;
    (t8662, t8668, t8676, t8679)
}

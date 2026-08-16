//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1317/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1317(t100026: f64, t100029: f64, t100031: f64, t100033: f64, t100034: f64, t100903: f64, t100927: f64, t101615: f64, t101735: f64, t187: f64, t99837: f64, t99839: f64, t99842: f64, t99845: f64, t99847: f64, t99850: f64, t99852: f64, t99854: f64, t99856: f64, t99859: f64, t99861: f64, t99864: f64) -> f64 {
    let t101739 = -t99837 + t99839 + t99842 + t99845 + t99847 + t99850 + t99852 + t99854 + t99856 + t99859 + t99861 - t99864 - t100026 - t100029 - t100031 + t100033 - t100034 + t187 * (t100903 + t100927 + t101615 + t101735);
    t101739
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1188/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1188(t19847: f64, t19850: f64, t19852: f64, t19854: f64, t19858: f64, t19860: f64, t19863: f64, t19866: f64, t19868: f64, t19871: f64, t19873: f64, t19875: f64, t19877: f64, t19880: f64, t19883: f64, t19886: f64, t19888: f64, t19892: f64) -> f64 {
    let t19894 = -t19847 / 288.0_f64 + t19850 / 96.0_f64 + t19852 / 48.0_f64 + 2.0_f64 / 9.0_f64 * t19854 - t19858 / 48.0_f64 - t19860 / 12.0_f64 + t19863 / 36.0_f64 - t19866 / 128.0_f64 + t19868 / 24.0_f64 - t19871 / 24.0_f64 - t19873 / 12.0_f64 + t19875 / 3.0_f64 + t19877 / 96.0_f64 - t19880 / 72.0_f64 + 3.0_f64 / 128.0_f64 * t19883 + t19886 / 24.0_f64 - t19888 / 6.0_f64 + t19892 / 36.0_f64;
    t19894
}

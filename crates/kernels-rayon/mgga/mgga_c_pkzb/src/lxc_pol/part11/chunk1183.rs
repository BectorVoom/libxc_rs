//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1183/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1183(t24: f64, t11146: f64, t11150: f64, t1165: f64, t1430: f64, t28895: f64, t28898: f64, t28906: f64, t3019: f64, t3022: f64, t333: f64, t3725: f64, t507: f64, t8742: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t29065 = piecewise3(t90, 0.0_f64, -56.0_f64 / 81.0_f64 * t11146 * t507 - 16.0_f64 / 9.0_f64 * t3725 * t1430 + 8.0_f64 / 9.0_f64 * t3019 * t28895 + 4.0_f64 / 3.0_f64 * t3022 * t28898 - 2.0_f64 / 3.0_f64 * t1165 * t8742 - 2.0_f64 / 9.0_f64 * t11150 * t507 + 2.0_f64 / 3.0_f64 * t333 * t28906);
    t29065
}

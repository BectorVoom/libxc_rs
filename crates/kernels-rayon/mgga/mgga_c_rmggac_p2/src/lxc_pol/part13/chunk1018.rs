//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1018/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1018(t8340: f64, t7213: f64, t7245: f64, t8021: f64, t8022: f64, t8023: f64, t8024: f64, t8025: f64, t8027: f64, t8028: f64, t8029: f64, t8344: f64) -> (f64, f64) {
    let t42369 = 0.13637330827122670865e-1_f64 * t8340;
    let t42370 = -0.325201597776800302e-2_f64 * t7213 + t8021 + t8022 + t8023 + t8024 - t8025 + 0.79453919800822633544e-4_f64 * t7245 - t8027 - t8028 - t8029 + t42369;
    let t42372 = 0.1440846329149835838e-2_f64 * t8344;
    (t42370, t42372)
}

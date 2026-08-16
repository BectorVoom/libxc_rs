//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2012/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2012(t1843: f64, t90551: f64, t90581: f64, t90582: f64, t93313: f64, t93359: f64, t93361: f64, t93362: f64, t96910: f64, t96920: f64, t96925: f64, t96929: f64) -> f64 {
    let t102493 = t93359 - 0.9869604401089358619e-1_f64 * t96910 + t93361 - 2.0_f64 * t93313 * t1843 - t93362 - 0.15352717957250113407e0_f64 * t96920 - 0.9869604401089358619e-1_f64 * t96925 + 0.19739208802178717238e0_f64 * t96929 - 0.20835831513410868196e0_f64 * t90551 - t90581 + 0.10417915756705434098e0_f64 * t90582;
    t102493
}

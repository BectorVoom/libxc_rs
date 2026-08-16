//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2121/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121(t109: f64, t81438: f64, t81440: f64, t86589: f64, t86591: f64, t92121: f64, t96713: f64, t96716: f64, t96719: f64, t96721: f64, t96724: f64, t96726: f64, t1268: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t96728 = -t81438 - 11.0_f64 / 9.0_f64 * t81440 - t92121 - t86589 + t86591 - 2.0_f64 / 3.0_f64 * t96713 - 3.0_f64 / 4.0_f64 * t96716 + t96719 / 2.0_f64 + t96721 / 3.0_f64 + t96724 / 4.0_f64 - t96726 / 8.0_f64;
    let t96729 = piecewise3(t110, 0.0_f64, t96728);
    let t96731 = 2.0_f64 * t1268 * t96729;
    (t96729, t96731)
}

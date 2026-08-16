//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1957/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957(t109: f64, t84036: f64, t86583: f64, t86586: f64, t92122: f64, t92123: f64, t96713: f64, t96716: f64, t96719: f64, t96721: f64, t96724: f64, t96726: f64, t2098: f64, t671: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t100989 = -t84036 - t86583 - 44.0_f64 / 9.0_f64 * t86586 - t92122 + t92123 - 4.0_f64 / 3.0_f64 * t96713 - 3.0_f64 / 2.0_f64 * t96716 + t96719 + 2.0_f64 / 3.0_f64 * t96721 + t96724 / 2.0_f64 - t96726 / 4.0_f64;
    let t100990 = piecewise3(t110, 0.0_f64, t100989);
    let t100993 = t2098 * t671;
    (t100990, t100993)
}

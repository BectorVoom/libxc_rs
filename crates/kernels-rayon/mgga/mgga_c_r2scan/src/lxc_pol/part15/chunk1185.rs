//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1185/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1185(t40197: f64, t40201: f64, t40204: f64, t40207: f64, t40210: f64, t40213: f64, t40216: f64, t40218: f64, t40220: f64, t40223: f64, t40224: f64, t40228: f64) -> f64 {
    let t40230 = -0.32927245914677557994e0_f64 * t40197 - 0.95219938395347901943e-2_f64 * t40201 + 0.2600466522016280569e0_f64 * t40204 - 0.2600466522016280569e0_f64 * t40207 + 0.10975748638225852664e0_f64 * t40210 - 0.10401866088065122276e1_f64 * t40213 - t40216 - t40218 + 0.22511059664845582436e0_f64 * t40220 - t40223 - 0.43663693315433241792e-2_f64 * t40224 + 0.16262400898971305031e-3_f64 * t40228;
    t40230
}

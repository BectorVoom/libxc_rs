//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 614/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk614<F: Float>(t1175: F, t372: F, t1165: F, t1552: F, t1532: F, t3196: F, t1539: F, t301: F, t3194: F, t129: F, t145: F, t4875: F, t5: F, t1173: F, t127: F, t3816: F, t418: F, t5253: F, t5255: F, t5260: F, t5263: F, t5267: F, t5272: F) -> (F, F, F, F, F, F) {
    let t5275 = t1175 * t372;
    let t5277 = t1165 * t1552 * t5275;
    let t5281 = t1165 * t1532 * t3196;
    let t5284 = t1539 * t301;
    let t5286 = t1165 * t1532 * t5284;
    let t5288 = 0.17149607247227894789e-2 * t3194 * t5286;
    let t5291 = t129 * t5 * t4875 * t145;
    let t5294 = 35.0 / 216.0 * t3816 + t5253 + 0.85748036236139473944e-2 * t418 * t5255 - 0.34299214494455789578e-2 * t418 * t5260 - 0.80031500487063509014e-2 * t5263 - 0.85748036236139473945e-2 * t418 * t5267 + 0.34299214494455789578e-2 * t1173 * t5272 - 0.34299214494455789578e-2 * t1173 * t5277 + 0.17149607247227894789e-2 * t1173 * t5281 - t5288 + t127 * t5291 / 96.0;
    (t5275, t5277, t5281, t5284, t5286, t5294)
}

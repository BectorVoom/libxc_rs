//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 655/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk655<F: Float>(t1539: F, t301: F, t1165: F, t1532: F, t3194: F, t129: F, t145: F, t4875: F, t5: F, t1173: F, t127: F, t3816: F, t418: F, t5253: F, t5255: F, t5260: F, t5263: F, t5267: F, t5272: F, t5277: F, t5281: F) -> (F, F, F) {
    let t5284 = t1539 * t301;
    let t5286 = t1165 * t1532 * t5284;
    let t5288 = F::cast_from(0.17149607247227894789e-2_f64) * t3194 * t5286;
    let t5291 = t129 * t5 * t4875 * t145;
    let t5294 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t3816 + t5253 + F::cast_from(0.85748036236139473944e-2_f64) * t418 * t5255 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t5260 - F::cast_from(0.80031500487063509014e-2_f64) * t5263 - F::cast_from(0.85748036236139473945e-2_f64) * t418 * t5267 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t5272 - F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t5277 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t5281 - t5288 + t127 * t5291 / F::cast_from(96.0_f64);
    (t5284, t5286, t5294)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1980/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1980(t98218: f64, t98220: f64, t98222: f64, t98224: f64, t98226: f64, t98229: f64, t94479: f64, t96323: f64, t98211: f64, t98213: f64, t98215: f64, t98231: f64) -> f64 {
    let t102526 = 0.1219527626469539185e-2_f64 * t98218;
    let t102527 = 0.18071592998981862717e-4_f64 * t98220;
    let t102528 = 0.16006300097412701803e0_f64 * t98222;
    let t102529 = 0.22675591804667994221e-1_f64 * t98224;
    let t102530 = 0.80031500487063509014e-2_f64 * t98226;
    let t102531 = 0.22866142996303859718e-3_f64 * t98229;
    let t102533 = 0.34299214494455789578e-2_f64 * t98211 - 0.85748036236139473944e-3_f64 * t98213 + 0.34299214494455789578e-2_f64 * t98215 - t96323 + 0.81312004494856525159e-4_f64 * t94479 - t102526 - t102527 - t102528 - t102529 + t102530 - t102531 + 0.17149607247227894789e-1_f64 * t98231;
    t102533
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 765/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk765(t7631: f64, t7638: f64, t7640: f64, t7671: f64, t7673: f64, t7622: f64, t7624: f64, t7626: f64, t7628: f64, t7644: f64, t7648: f64, t7650: f64, t7652: f64, t7654: f64, t7659: f64, t7661: f64, t7663: f64, t7665: f64, t7667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8219 = 0.37737710747524982482e-2_f64 * t7631;
    let t8220 = 0.27953859812981468505e-2_f64 * t7638;
    let t8221 = 0.25724410870841842184e-2_f64 * t7640;
    let t8232 = 0.42874018118069736972e-3_f64 * t7671;
    let t8233 = 13.0_f64 / 144.0_f64 * t7673;
    let t8234 = 0.16006300097412701803e-1_f64 * t7622 - 0.68598428988911579156e-2_f64 * t7624 - 0.34299214494455789578e-2_f64 * t7626 + 0.34299214494455789578e-2_f64 * t7628 + t8219 + t8220 - t8221 + 0.21437009059034868486e-2_f64 * t7644 + 0.17149607247227894789e-2_f64 * t7648 + 0.68598428988911579156e-2_f64 * t7650 - 0.34299214494455789578e-2_f64 * t7652 + 0.34299214494455789578e-2_f64 * t7654 - 0.94344276868812456204e-2_f64 * t7659 - 0.68598428988911579156e-2_f64 * t7661 - 0.13719685797782315831e-1_f64 * t7663 + 0.13719685797782315831e-1_f64 * t7665 - 0.85748036236139473944e-3_f64 * t7667 + t8232 - t8233;
    (t8219, t8220, t8221, t8232, t8233, t8234)
}

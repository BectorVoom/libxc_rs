//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1978/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1978(t98161: f64, t98165: f64, t98168: f64, t98180: f64, t94456: f64, t94460: f64, t98170: f64, t98172: f64, t98174: f64, t98176: f64, t98178: f64, t98182: f64) -> f64 {
    let t102495 = 0.10164000561857065645e-4_f64 * t98161;
    let t102498 = 0.90702367218671976884e-1_f64 * t98165;
    let t102499 = 7.0_f64 / 12.0_f64 * t98168;
    let t102505 = 0.10164000561857065645e-3_f64 * t98180;
    let t102507 = t102495 - 0.80031500487063509014e-2_f64 * t94456 - 0.45351183609335988442e-1_f64 * t94460 - t102498 - t102499 + t98170 / 4.0_f64 + t98172 / 8.0_f64 + 0.54208002996571016773e-3_f64 * t98174 - 0.10289764348336736873e0_f64 * t98176 - 0.85748036236139473944e-3_f64 * t98178 - t102505 - 0.17149607247227894789e-2_f64 * t98182;
    t102507
}

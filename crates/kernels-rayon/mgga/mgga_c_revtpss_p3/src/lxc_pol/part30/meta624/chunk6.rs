//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2156/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2156(t14668: f64, t27261: f64, t14933: f64, t2482: f64, t25260: f64, t814: f64, t2689: f64, t27239: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93043: f64, t93045: f64, t93049: f64, t93055: f64, t93058: f64) -> f64 {
    let t99081 = t27261 * t14668;
    let t99085 = t2482 * t25260 * t814 * t14933;
    let t99086 = 0.10164000561857065645e-3_f64 * t99085;
    let t99091 = t2689 * t27239;
    let t99098 = 0.85748036236139473944e-3_f64 * t99081 + t99086 + 0.50820002809285328226e-4_f64 * t93026 + 0.20007875121765877254e-2_f64 * t93028 - 0.11433071498151929859e-3_f64 * t93031 + 0.54208002996571016774e-3_f64 * t93035 - 0.60976381323476959249e-3_f64 * t99091 - 0.25410001404642664113e-4_f64 * t93043 + 0.20007875121765877254e-2_f64 * t93045 - 0.22675591804667994222e-1_f64 * t93049 - 0.40015750243531754508e-2_f64 * t93055 - 0.25410001404642664113e-4_f64 * t93058;
    t99098
}

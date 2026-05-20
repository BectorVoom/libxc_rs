//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2156/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2156<F: Float>(t14668: F, t27261: F, t14933: F, t2482: F, t25260: F, t814: F, t2689: F, t27239: F, t93026: F, t93028: F, t93031: F, t93035: F, t93043: F, t93045: F, t93049: F, t93055: F, t93058: F) -> F {
    let t99081 = t27261 * t14668;
    let t99085 = t2482 * t25260 * t814 * t14933;
    let t99086 = F::cast_from(0.10164000561857065645e-3_f64) * t99085;
    let t99091 = t2689 * t27239;
    let t99098 = F::cast_from(0.85748036236139473944e-3_f64) * t99081 + t99086 + F::cast_from(0.50820002809285328226e-4_f64) * t93026 + F::cast_from(0.20007875121765877254e-2_f64) * t93028 - F::cast_from(0.11433071498151929859e-3_f64) * t93031 + F::cast_from(0.54208002996571016774e-3_f64) * t93035 - F::cast_from(0.60976381323476959249e-3_f64) * t99091 - F::cast_from(0.25410001404642664113e-4_f64) * t93043 + F::cast_from(0.20007875121765877254e-2_f64) * t93045 - F::cast_from(0.22675591804667994222e-1_f64) * t93049 - F::cast_from(0.40015750243531754508e-2_f64) * t93055 - F::cast_from(0.25410001404642664113e-4_f64) * t93058;
    t99098
}

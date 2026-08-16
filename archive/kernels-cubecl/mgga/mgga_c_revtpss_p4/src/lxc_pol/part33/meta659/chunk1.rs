//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2129/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2129<F: Float>(t1955: F, t27212: F, t5977: F, t7048: F, t18784: F, t1949: F, t231: F, t25383: F, t27199: F, t27300: F, t27353: F, t27358: F, t29655: F, t29691: F, t62589: F, t7070: F, t7071: F, t7076: F, t93175: F, t93177: F, t99174: F, t99212: F, t99216: F, t99219: F, t99222: F, t99228: F, t99231: F) -> (F, F, F) {
    let t106172 = t1955 * t27212;
    let t106178 = t7048 * t5977;
    let t106190 = F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t1949 * t18784 - F::cast_from(0.68540937416128198419e-2_f64) * t99212 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t29655 + t99216 + t99219 - F::cast_from(0.17347256376410398924e1_f64) * t106172 * t27358 - t99222 - F::cast_from(0.17135234354032049604e-2_f64) * t93175 + F::cast_from(0.4336814094102599731e0_f64) * t25383 * t29691 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t106178 * t231 - F::cast_from(0.52041769129231196772e1_f64) * t27199 * t27300 + F::cast_from(0.26020884564615598386e1_f64) * t27353 * t99174 * t62589 - F::cast_from(0.22849835011101738147e-2_f64) * t93177 + F::cast_from(0.39029762157531132076e-1_f64) * t99228 + t99231;
    (t106172, t106178, t106190)
}

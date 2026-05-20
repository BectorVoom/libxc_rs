//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2173/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2173<F: Float>(t1568: F, t786: F, t25410: F, t25413: F, t14587: F, t25383: F, t25391: F, t2718: F, t27189: F, t27287: F, t27292: F, t27300: F, t27312: F, t27313: F, t27353: F, t27357: F, t2829: F, t51574: F, t7048: F, t92864: F, t92917: F, t93297: F, t93304: F, t99369: F, t99375: F, t99381: F, t99391: F) -> (F, F) {
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99406 = F::cast_from(0.14456046980341999104e-1_f64) * t99404 * t25413;
    let t99409 = F::cast_from(0.17347256376410398924e1_f64) * t25391 * t27357 * t99369 - t99375 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t27357 * t51574 + F::cast_from(0.17135234354032049604e-2_f64) * t99381 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t27287 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t27292 - F::cast_from(0.52041769129231196772e1_f64) * t25383 * t27300 - t99391 - F::cast_from(0.25702851531048074406e-1_f64) * t93297 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t2718 * t7048 * t14587 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t27313 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t92864 * t27312 - F::cast_from(0.25702851531048074406e-1_f64) * t93304 - t99406 - F::cast_from(0.65854491829355115987e0_f64) * t27189 * t2829;
    (t99403, t99409)
}

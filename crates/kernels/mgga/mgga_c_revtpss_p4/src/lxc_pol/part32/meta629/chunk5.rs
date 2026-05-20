//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2024/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2024<F: Float>(t30395: F, t689: F, t25431: F, t25411: F, t6072: F, t7384: F, t5977: F, t7398: F, t103037: F, t103224: F, t103234: F, t103240: F, t103364: F, t106228: F, t18324: F, t25383: F, t25391: F, t25416: F, t2723: F, t27275: F, t27349: F, t28425: F, t30392: F, t7070: F, t7403: F, t8016: F, t93349: F, t95836: F) -> (F, F) {
    let t110475 = t30395 * t689;
    let t110476 = t25431 * t110475;
    let t110478 = t25411 * t110475;
    let t110489 = t689 * t7384 * t6072;
    let t110493 = t7398 * t5977;
    let t110499 = -t103224 - F::cast_from(0.4818682326780666368e-3_f64) * t103234 + F::cast_from(0.52041769129231196772e1_f64) * t93349 * t103037 * t27349 - F::cast_from(0.45699670022203476294e-2_f64) * t103240 - F::cast_from(0.72280234901709995518e-2_f64) * t110476 + F::cast_from(0.12851425765524037203e-1_f64) * t110478 - F::cast_from(0.17135234354032049604e-2_f64) * t95836 - F::cast_from(0.8673628188205199462e0_f64) * t27275 * t8016 + F::cast_from(0.17347256376410398924e1_f64) * t25391 * t28425 * t106228 + F::cast_from(0.13170898365871023197e1_f64) * t7403 * t18324 + F::cast_from(0.54878743191129263322e-2_f64) * t110489 - F::cast_from(0.8673628188205199462e0_f64) * t25383 * t30392 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25416 * t110493 * t2723 + F::cast_from(0.3427046870806409921e-2_f64) * t103364;
    (t110493, t110499)
}

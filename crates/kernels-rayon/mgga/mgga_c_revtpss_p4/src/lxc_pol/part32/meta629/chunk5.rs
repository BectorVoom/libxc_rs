//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2024/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2024(t30395: f64, t689: f64, t25431: f64, t25411: f64, t6072: f64, t7384: f64, t5977: f64, t7398: f64, t103037: f64, t103224: f64, t103234: f64, t103240: f64, t103364: f64, t106228: f64, t18324: f64, t25383: f64, t25391: f64, t25416: f64, t2723: f64, t27275: f64, t27349: f64, t28425: f64, t30392: f64, t7070: f64, t7403: f64, t8016: f64, t93349: f64, t95836: f64) -> (f64, f64) {
    let t110475 = t30395 * t689;
    let t110476 = t25431 * t110475;
    let t110478 = t25411 * t110475;
    let t110489 = t689 * t7384 * t6072;
    let t110493 = t7398 * t5977;
    let t110499 = -t103224 - 0.4818682326780666368e-3_f64 * t103234 + 0.52041769129231196772e1_f64 * t93349 * t103037 * t27349 - 0.45699670022203476294e-2_f64 * t103240 - 0.72280234901709995518e-2_f64 * t110476 + 0.12851425765524037203e-1_f64 * t110478 - 0.17135234354032049604e-2_f64 * t95836 - 0.8673628188205199462e0_f64 * t27275 * t8016 + 0.17347256376410398924e1_f64 * t25391 * t28425 * t106228 + 0.13170898365871023197e1_f64 * t7403 * t18324 + 0.54878743191129263322e-2_f64 * t110489 - 0.8673628188205199462e0_f64 * t25383 * t30392 - 0.8673628188205199462e0_f64 * t7070 * t25416 * t110493 * t2723 + 0.3427046870806409921e-2_f64 * t103364;
    (t110493, t110499)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1208/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1208(t127703: f64, t867: f64, t7060: f64, t7063: f64, t127724: f64, t32469: f64, t27279: f64, t32481: f64, t120057: f64, t121846: f64, t121975: f64, t126280: f64, t1579: f64, t27207: f64, t27291: f64, t27313: f64, t27349: f64, t28425: f64, t31812: f64, t32426: f64, t32429: f64, t32434: f64, t32463: f64, t34069: f64, t4533: f64, t8649: f64, t8651: f64) -> (f64, f64) {
    let t127767 = t127703 * t867;
    let t127769 = t7063 * t127767 * t7060;
    let t127774 = t32469 * t127724;
    let t127776 = t32481 * t27279;
    let t127788 = -0.17135921299530705785e1_f64 * t32426 * t34069 - 0.17135921299530705785e1_f64 * t8649 * t31812 * t8651 * t4533 + 0.8673628188205199462e0_f64 * t32434 * t27207 - 0.25702851531048074406e-1_f64 * t127769 - 0.11423947533020470523e1_f64 * t32463 * t28425 * t27291 - 0.14279934416275588154e-1_f64 * t127774 - 0.25702851531048074406e-1_f64 * t127776 + 0.34271842599061411569e1_f64 * t120057 * t121846 * t27349 - 0.17347256376410398924e1_f64 * t121975 * t27313 + 0.112937867033921868e-2_f64 * t126280 - 0.17135921299530705785e1_f64 * t8649 * t31812 * t32429 * t1579;
    (t127767, t127788)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1211/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1211<F: Float>(t127703: F, t867: F, t7060: F, t7063: F, t127724: F, t32469: F, t27279: F, t32481: F, t120057: F, t121846: F, t121975: F, t126280: F, t1579: F, t27207: F, t27291: F, t27313: F, t27349: F, t28425: F, t31812: F, t32426: F, t32429: F, t32434: F, t32463: F, t34069: F, t4533: F, t8649: F, t8651: F) -> (F, F) {
    let t127767 = t127703 * t867;
    let t127769 = t7063 * t127767 * t7060;
    let t127774 = t32469 * t127724;
    let t127776 = t32481 * t27279;
    let t127788 = -F::cast_from(0.17135921299530705785e1_f64) * t32426 * t34069 - F::cast_from(0.17135921299530705785e1_f64) * t8649 * t31812 * t8651 * t4533 + F::cast_from(0.8673628188205199462e0_f64) * t32434 * t27207 - F::cast_from(0.25702851531048074406e-1_f64) * t127769 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t28425 * t27291 - F::cast_from(0.14279934416275588154e-1_f64) * t127774 - F::cast_from(0.25702851531048074406e-1_f64) * t127776 + F::cast_from(0.34271842599061411569e1_f64) * t120057 * t121846 * t27349 - F::cast_from(0.17347256376410398924e1_f64) * t121975 * t27313 + F::cast_from(0.112937867033921868e-2_f64) * t126280 - F::cast_from(0.17135921299530705785e1_f64) * t8649 * t31812 * t32429 * t1579;
    (t127767, t127788)
}

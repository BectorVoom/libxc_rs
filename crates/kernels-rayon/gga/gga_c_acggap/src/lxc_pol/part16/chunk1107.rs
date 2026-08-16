//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1107/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1107(t30591: f64, t30592: f64, t30596: f64, t30601: f64, t30605: f64, t30613: f64, t34435: f64, t34449: f64, t34478: f64, t34489: f64, t34492: f64, t37105: f64, t37112: f64, t37114: f64, t39330: f64, t39334: f64, t39337: f64, t39343: f64) -> f64 {
    let t39351 = 0.114609375e-1_f64 * t39330 + 0.114609375e-1_f64 * t39334 + 0.7640625e-2_f64 * t39337 + 0.94344276868812456204e-3_f64 * t34435 + t30591 + 0.47637797908966374414e-2_f64 * t30592 - 0.7862023072401038017e-3_f64 * t39343 + 0.12579236915841660827e-2_f64 * t34449 + t37105 - t37112 + t37114 + 0.11321313224257494744e-1_f64 * t34478 + t30596 - t30601 / 128.0_f64 - t30605 / 384.0_f64 + t34489 - 0.31448092289604152068e-3_f64 * t34492 - 0.12862205435420921092e-2_f64 * t30613;
    t39351
}

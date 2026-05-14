//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 982/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk982<F: Float>(t2030: F, t361: F, t9700: F, t142: F, t5506: F, t599: F, t2060: F, t9704: F, t1165: F, t5969: F, t604: F, t7493: F, t30591: F, t30592: F, t30596: F, t30601: F, t30605: F, t30613: F, t34435: F, t34449: F, t34478: F, t34489: F, t34492: F, t37105: F, t37112: F, t37114: F) -> (F,) {
    let t39330 = t2030 * t361 * t9700;
    let t39334 = t2030 * t142 * t599 * t5506;
    let t39337 = t2060 * t361 * t9704;
    let t39343 = t7493 * t1165 * t604 * t5969;
    let t39351 = 0.114609375e-1 * t39330 + 0.114609375e-1 * t39334 + 0.7640625e-2 * t39337 + 0.94344276868812456204e-3 * t34435 + t30591 + 0.47637797908966374414e-2 * t30592 - 0.7862023072401038017e-3 * t39343 + 0.12579236915841660827e-2 * t34449 + t37105 - t37112 + t37114 + 0.11321313224257494744e-1 * t34478 + t30596 - t30601 / 128.0 - t30605 / 384.0 + t34489 - 0.31448092289604152068e-3 * t34492 - 0.12862205435420921092e-2 * t30613;
    (t39351,)
}

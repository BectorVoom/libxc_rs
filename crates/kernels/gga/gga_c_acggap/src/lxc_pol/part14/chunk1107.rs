//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1107/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1107<F: Float>(t30591: F, t30592: F, t30596: F, t30601: F, t30605: F, t30613: F, t34435: F, t34449: F, t34478: F, t34489: F, t34492: F, t37105: F, t37112: F, t37114: F, t39330: F, t39334: F, t39337: F, t39343: F) -> F {
    let t39351 = F::cast_from(0.114609375e-1_f64) * t39330 + F::cast_from(0.114609375e-1_f64) * t39334 + F::new(0.7640625e-2) * t39337 + F::cast_from(0.94344276868812456204e-3_f64) * t34435 + t30591 + F::cast_from(0.47637797908966374414e-2_f64) * t30592 - F::cast_from(0.7862023072401038017e-3_f64) * t39343 + F::cast_from(0.12579236915841660827e-2_f64) * t34449 + t37105 - t37112 + t37114 + F::cast_from(0.11321313224257494744e-1_f64) * t34478 + t30596 - t30601 / F::new(128.0) - t30605 / F::new(384.0) + t34489 - F::cast_from(0.31448092289604152068e-3_f64) * t34492 - F::cast_from(0.12862205435420921092e-2_f64) * t30613;
    t39351
}

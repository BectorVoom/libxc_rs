//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1377/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1377<F: Float>(t127429: F, t31535: F, t127542: F, t285: F, t800: F, t1200: F, t123226: F, t123676: F, t127262: F, t127278: F, t127389: F, t127421: F, t127426: F, t127438: F, t127519: F, t127539: F, t127545: F, t127595: F, t1472: F, t14721: F, t14742: F, t14766: F, t28603: F, t28695: F, t31489: F, t31508: F, t4104: F, t70550: F, t706: F) -> (F,) {
    let t127697 = t31535 * t127429;
    let t127700 = t285 * t127542;
    let t127703 = t800 * t127429;
    let t127720 = t1200 * t127429;
    let t127723 = -0.45306850413028723348e0 * t28695 * t31508 - 0.24163653553615319118e1 * t1472 * t127595 + 0.48327307107230638237e1 * t28695 * t31489 + 0.48327307107230638237e1 * t4104 * t127519 - 0.47085742397875932523e-2 * t127697 * t706 + 0.56502890877451119026e-1 * t127700 * t127545 - 0.56502890877451119026e-1 * t127703 * t127539 - 0.45306850413028723348e0 * t14721 * t127426 + 0.45306850413028723348e0 * t14742 * t127262 - 0.46992870109762241323e0 * t28603 * t123676 + 0.90613700826057446696e0 * t14766 * t127438 - 0.48327307107230638237e1 * t14766 * t127421 + 0.21895580739717983994e1 * t70550 * t127278 + 0.21895580739717983994e1 * t70550 * t127389 + 0.18834296959150373009e-1 * t127720 * t123226;
    (t127723,)
}

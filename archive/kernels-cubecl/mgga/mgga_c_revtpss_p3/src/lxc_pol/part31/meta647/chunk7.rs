//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2131/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2131<F: Float>(t27341: F, t99463: F, t99466: F, t14495: F, t25391: F, t27199: F, t27287: F, t29659: F, t4533: F, t7067: F, t7070: F, t7071: F, t7759: F, t93372: F, t99414: F, t99460: F, t99465: F, t99468: F, t99472: F, t99475: F, t99480: F, t99481: F) -> F {
    let t106446 = t99463 * t27341;
    let t106448 = t99466 * t27341;
    let t106461 = -F::cast_from(0.4336814094102599731e0_f64) * t7067 * t29659 + F::cast_from(0.91399340044406952588e-2_f64) * t99460 + F::cast_from(0.51405703062096148813e-1_f64) * t106446 + t99465 - t99468 - F::cast_from(0.28912093960683998207e-1_f64) * t106448 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t27287 - t99472 + t99475 + F::cast_from(0.22849835011101738147e-2_f64) * t93372 - t99480 - F::cast_from(0.19274729307122665472e-1_f64) * t99481 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t7759 * t4533 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t99414 * t14495;
    t106461
}

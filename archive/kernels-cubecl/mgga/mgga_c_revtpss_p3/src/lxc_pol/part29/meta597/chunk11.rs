//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2026/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2026<F: Float>(t2435: F, t28448: F, t28314: F, t93364: F, t103483: F, t231: F, t25391: F, t25416: F, t2645: F, t26489: F, t26550: F, t27199: F, t2723: F, t4533: F, t7070: F, t7071: F, t7076: F, t7398: F, t7997: F, t8007: F, t93126: F, t95902: F, t95905: F, t95911: F, t95914: F, t95925: F, t95927: F, t99360: F) -> F {
    let t103490 = t2435 * t28448;
    let t103494 = F::cast_from(0.28912093960683998208e-1_f64) * t93364 * t28314;
    let t103519 = F::cast_from(0.73171657588172351096e-2_f64) * t103490 + F::cast_from(0.91399340044406952588e-2_f64) * t95902 - t103494 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25416 * t103483 * t2723 - F::cast_from(0.14634331517634470219e-1_f64) * t95905 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t26550 * t99360 - F::cast_from(0.26020884564615598386e1_f64) * t27199 * t26489 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t7997 * t2645 * t231 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t7398 * t4533 + F::cast_from(0.8673628188205199462e0_f64) * t93126 * t8007 + F::cast_from(0.96373646535613327358e-3_f64) * t95911 + t95914 + F::cast_from(0.13009920719177044025e-2_f64) * t95925 - F::cast_from(0.2601984143835408805e-1_f64) * t95927;
    t103519
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2026/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2026(t2435: f64, t28448: f64, t28314: f64, t93364: f64, t103483: f64, t231: f64, t25391: f64, t25416: f64, t2645: f64, t26489: f64, t26550: f64, t27199: f64, t2723: f64, t4533: f64, t7070: f64, t7071: f64, t7076: f64, t7398: f64, t7997: f64, t8007: f64, t93126: f64, t95902: f64, t95905: f64, t95911: f64, t95914: f64, t95925: f64, t95927: f64, t99360: f64) -> f64 {
    let t103490 = t2435 * t28448;
    let t103494 = 0.28912093960683998208e-1_f64 * t93364 * t28314;
    let t103519 = 0.73171657588172351096e-2_f64 * t103490 + 0.91399340044406952588e-2_f64 * t95902 - t103494 - 0.8673628188205199462e0_f64 * t7070 * t25416 * t103483 * t2723 - 0.14634331517634470219e-1_f64 * t95905 - 0.17347256376410398924e1_f64 * t25391 * t26550 * t99360 - 0.26020884564615598386e1_f64 * t27199 * t26489 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t7997 * t2645 * t231 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t7398 * t4533 + 0.8673628188205199462e0_f64 * t93126 * t8007 + 0.96373646535613327358e-3_f64 * t95911 + t95914 + 0.13009920719177044025e-2_f64 * t95925 - 0.2601984143835408805e-1_f64 * t95927;
    t103519
}

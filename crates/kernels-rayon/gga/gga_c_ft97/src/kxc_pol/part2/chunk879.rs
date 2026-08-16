//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 879/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk879(t238: f64, t13395: f64, t13400: f64, t13402: f64, t13407: f64, t13413: f64, t13414: f64, t13417: f64, t13422: f64, t13426: f64, t13429: f64, t13435: f64, t13489: f64, t13491: f64, t13495: f64, t13499: f64, t13502: f64, t13505: f64, t13509: f64, t13516: f64, t13520: f64, t13523: f64, t13527: f64, t13669: f64, t1417: f64, t1701: f64, t2387: f64, t2389: f64, t2428: f64, t3723: f64, t3759: f64, t3760: f64, t3766: f64, t3774: f64, t3776: f64, t3789: f64, t678: f64) -> f64 {
    let t239 = 0.1e-59_f64 < t238;
    let t13672 = piecewise3(t239, 0.13784064983740990796e-4_f64 * t3774 * t3776 * t13395 + 0.45915205659928668025e-5_f64 * t3774 * t13400 * t13402 - 0.68920324918704953981e-4_f64 * t3774 * t3776 * t13407 + 0.16027353291807919743e-5_f64 * t13413 * t13414 - 0.59273806478425129876e-2_f64 * t1417 * t1701 * t13417 - 0.46509801892875584e-1_f64 * t13422 * t2389 + 0.23254900946437792e-1_f64 * t2387 * t13426 + 0.38731446812548799881e-3_f64 * t3759 * t13429 * t13402 - 0.32253953169881963531e-5_f64 * t678 * t13435 - 0.23254900946437792e-1_f64 * t3759 * t3760 * t13407 + t13489 + 4.0_f64 * t3766 * t13491 * t2428 - 6.0_f64 * t3789 * t13495 * t2428 + 0.19365723406274399941e-3_f64 * t678 * t13499 + 0.38731446812548799882e-3_f64 * t678 * t13502 + 0.11627450473218896e-1_f64 * t2387 * t13505 - 0.19365723406274399941e-3_f64 * t2387 * t13509 + 0.46509801892875584e-2_f64 * t3759 * t3760 * t13395 + 0.23254900946437792e-1_f64 * t2387 * t13516 + 0.13784064983740990796e-3_f64 * t13520 * t13523 + 0.13519760450715832853e-3_f64 * t3723 * t13527 + t13669, 0.0_f64);
    t13672
}

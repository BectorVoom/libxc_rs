//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 879/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk879<F: Float>(t238: F, t13395: F, t13400: F, t13402: F, t13407: F, t13413: F, t13414: F, t13417: F, t13422: F, t13426: F, t13429: F, t13435: F, t13489: F, t13491: F, t13495: F, t13499: F, t13502: F, t13505: F, t13509: F, t13516: F, t13520: F, t13523: F, t13527: F, t13669: F, t1417: F, t1701: F, t2387: F, t2389: F, t2428: F, t3723: F, t3759: F, t3760: F, t3766: F, t3774: F, t3776: F, t3789: F, t678: F) -> F {
    let t239 = F::new(0.1e-59) < t238;
    let t13672 = piecewise3::<f64>(t239, F::new(0.13784064983740990796e-4) * t3774 * t3776 * t13395 + F::new(0.45915205659928668025e-5) * t3774 * t13400 * t13402 - F::new(0.68920324918704953981e-4) * t3774 * t3776 * t13407 + F::new(0.16027353291807919743e-5) * t13413 * t13414 - F::new(0.59273806478425129876e-2) * t1417 * t1701 * t13417 - F::new(0.46509801892875584e-1) * t13422 * t2389 + F::new(0.23254900946437792e-1) * t2387 * t13426 + F::new(0.38731446812548799881e-3) * t3759 * t13429 * t13402 - F::new(0.32253953169881963531e-5) * t678 * t13435 - F::new(0.23254900946437792e-1) * t3759 * t3760 * t13407 + t13489 + F::new(4.0) * t3766 * t13491 * t2428 - F::new(6.0) * t3789 * t13495 * t2428 + F::new(0.19365723406274399941e-3) * t678 * t13499 + F::new(0.38731446812548799882e-3) * t678 * t13502 + F::new(0.11627450473218896e-1) * t2387 * t13505 - F::new(0.19365723406274399941e-3) * t2387 * t13509 + F::new(0.46509801892875584e-2) * t3759 * t3760 * t13395 + F::new(0.23254900946437792e-1) * t2387 * t13516 + F::new(0.13784064983740990796e-3) * t13520 * t13523 + F::new(0.13519760450715832853e-3) * t3723 * t13527 + t13669, F::new(0.0));
    t13672
}

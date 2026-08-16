//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1118/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1118(t5157: f64, t7561: f64, t1165: f64, t22401: f64, t7351: f64, t7413: f64, t30817: f64, t8948: f64, t8793: f64, t4434: f64, t570: f64, t1313: f64, t30598: f64, t721: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35868 = t7561 * t5157;
    let t35872 = t7413 * t1165 * t7351 * t22401;
    let t35874 = t30817 * t8948;
    let t35876 = t30817 * t8793;
    let t35879 = t570 * t4434;
    let t35882 = t30598 * t1313 * t721;
    (t35868, t35872, t35874, t35876, t35879, t35882)
}

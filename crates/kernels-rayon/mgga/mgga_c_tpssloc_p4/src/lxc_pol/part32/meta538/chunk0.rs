//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1879/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1879(t27551: f64, t3961: f64, t27550: f64, t24826: f64, t8074: f64, t24788: f64, t8066: f64, t3247: f64, t491: f64, t24589: f64, t24845: f64, t24849: f64, t27533: f64, t27537: f64, t27540: f64, t27543: f64, t27546: f64, t27549: f64, t3604: f64, t3610: f64, t3624: f64, t7373: f64, t8083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27552 = t27551 * t3961;
    let t27553 = t27550 * t27552;
    let t27556 = t24826 * t8074;
    let t27558 = t24788 * t8066;
    let t27561 = t491 * t3247;
    let t27562 = t27561 * t3961;
    let t27563 = t27550 * t27562;
    let t27568 = -0.27415567780803773942e-2_f64 * t24849 * t27533 - 0.82246703342411321825e-2_f64 * t7373 * t27537 - 0.82246703342411321825e-2_f64 * t7373 * t27540 + 2.0_f64 * t3610 * t27543 - t3624 * t27546 + 0.36554090374405031923e-2_f64 * t27549 * t27553 + 0.27415567780803773942e-2_f64 * t27556 + 0.27415567780803773942e-2_f64 * t24589 * t27558 - 0.54831135561607547884e-2_f64 * t24589 * t27563 + 0.27415567780803773942e-2_f64 * t24845 + t3604 * t8083;
    (t27552, t27553, t27556, t27558, t27561, t27562, t27563, t27568)
}

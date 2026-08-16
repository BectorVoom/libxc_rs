//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2306/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306(t24667: f64, t6252: f64, t1653: f64, t8039: f64, t85822: f64, t6224: f64, t7348: f64, t24574: f64, t29741: f64, t29614: f64, t7327: f64, t103683: f64, t24589: f64, t24833: f64, t24858: f64, t27507: f64, t27520: f64, t27536: f64, t27537: f64, t27562: f64, t29781: f64, t3624: f64, t3625: f64, t5975: f64, t7283: f64, t7362: f64, t7373: f64, t7377: f64, t8066: f64, t8073: f64, t85820: f64, t86037: f64, t86102: f64, t94966: f64, t95803: f64, t95813: f64) -> (f64, f64) {
    let t103694 = t24667 * t6252;
    let t103699 = t85822 * t1653 * t8039;
    let t103707 = t7348 * t6224;
    let t103710 = t24574 * t29741;
    let t103723 = t29614 * t7327;
    let t103733 = 0.27415567780803773942e-2_f64 * t86037 * t103694 * t86102 + 0.54831135561607547884e-2_f64 * t85820 * t103699 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t24858 * t5975 + 0.12184696791468343974e-2_f64 * t94966 - t3624 * t103707 * t3625 - 0.18277045187202515961e-2_f64 * t103710 + 0.43864908449286038306e-1_f64 * t27507 * t27537 - 0.16449340668482264365e-1_f64 * t7373 * t24833 * t29781 + 0.54831135561607547884e-2_f64 * t24589 * t95813 * t8066 + 0.10966227112321509577e-1_f64 * t24589 * t103683 * t27562 - 0.82246703342411321825e-2_f64 * t7373 * t103723 * t7377 - 0.16449340668482264365e-1_f64 * t7373 * t95803 * t8073 - 0.16449340668482264365e-1_f64 * t7373 * t27536 * t27520;
    (t103707, t103733)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2306/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2306<F: Float>(t24667: F, t6252: F, t1653: F, t8039: F, t85822: F, t6224: F, t7348: F, t24574: F, t29741: F, t29614: F, t7327: F, t103683: F, t24589: F, t24833: F, t24858: F, t27507: F, t27520: F, t27536: F, t27537: F, t27562: F, t29781: F, t3624: F, t3625: F, t5975: F, t7283: F, t7362: F, t7373: F, t7377: F, t8066: F, t8073: F, t85820: F, t86037: F, t86102: F, t94966: F, t95803: F, t95813: F) -> (F, F) {
    let t103694 = t24667 * t6252;
    let t103699 = t85822 * t1653 * t8039;
    let t103707 = t7348 * t6224;
    let t103710 = t24574 * t29741;
    let t103723 = t29614 * t7327;
    let t103733 = F::cast_from(0.27415567780803773942e-2_f64) * t86037 * t103694 * t86102 + F::cast_from(0.54831135561607547884e-2_f64) * t85820 * t103699 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t24858 * t5975 + F::cast_from(0.12184696791468343974e-2_f64) * t94966 - t3624 * t103707 * t3625 - F::cast_from(0.18277045187202515961e-2_f64) * t103710 + F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t27537 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t24833 * t29781 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t95813 * t8066 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t103683 * t27562 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t103723 * t7377 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t95803 * t8073 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t27536 * t27520;
    (t103707, t103733)
}

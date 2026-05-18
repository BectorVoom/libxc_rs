//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 913/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk913<F: Float>(t13635: F, t23157: F, t11844: F, t2021: F, t7372: F, t2976: F, t44787: F, t900: F, t13625: F, t22665: F, t7427: F, t2536: F, t3601: F) -> (F, F, F, F, F) {
    let t45513 = t23157 * t13635;
    let t45516 = t2021 * t11844 * t7372;
    let t45517 = F::new(0.14896037479937677779e-1) * t45516;
    let t45519 = t2976 * t900 * t44787;
    let t45520 = F::new(0.29792074959875355558e-1) * t45519;
    let t45522 = t7427 * t22665 * t13625;
    let t45524 = t2536 * t3601;
    (t45513, t45517, t45520, t45522, t45524)
}

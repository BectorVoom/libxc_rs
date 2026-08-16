//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 661/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk661<F: Float>(t438: F, t4753: F, t1449: F, t430: F, t63: F, t1452: F, t71: F, t4728: F, t1450: F, t377: F, t124: F, t431: F) -> (F, F, F, F, F) {
    let t4754 = t4753 * t438;
    let t4758 = F::cast_from(1.0_f64) / t1449 / t430;
    let t4759 = t63 * t4758;
    let t4761 = F::cast_from(1.0_f64) / t1452 / t71;
    let t4762 = t4728 * t4761;
    let t4768 = t377 * t1450;
    let t4772 = t124 * t431;
    (t4754, t4759, t4762, t4768, t4772)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1066/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1066<F: Float>(t1156: F, t3476: F, t3475: F, t431: F, t426: F, t12295: F, t12351: F, t1159: F, t3478: F, t434: F, t1179: F, t3488: F) -> (F, F, F, F, F, F, F) {
    let t12423 = t1156 * t3476;
    let t12428 = F::cast_from(1.0_f64) / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12459 = F::cast_from(0.16068111111111111111e1_f64) * t12295;
    let t12460 = F::cast_from(0.46308888888888888888e0_f64) * t12351;
    let t12469 = F::cast_from(1.0_f64) / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = F::cast_from(1.0_f64) / t3478 / t434;
    let t12476 = t3488 * t1179;
    (t12423, t12429, t12459, t12460, t12470, t12472, t12476)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3094/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094<F: Float>(t12361: F, t24212: F, t3384: F, t5105: F, t6470: F, t24765: F, t3531: F, t1196: F, t16988: F, t20472: F, t1733: F, t20447: F) -> (F, F, F, F, F) {
    let t81601 = F::cast_from(6.0_f64) * t12361 * t24212;
    let t81604 = F::cast_from(6.0_f64) * t3384 * t5105 * t6470;
    let t81606 = F::cast_from(0.10254018858216406658e4_f64) * t3531 * t24765;
    let t81609 = F::cast_from(0.31168546390226634765e3_f64) * t1196 * t20472 * t16988;
    let t81612 = F::cast_from(6.0_f64) * t3384 * t1733 * t20447;
    (t81601, t81604, t81606, t81609, t81612)
}

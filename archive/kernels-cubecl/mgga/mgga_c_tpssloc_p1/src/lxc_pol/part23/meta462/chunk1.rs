//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1353/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353<F: Float>(t68500: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t76893: F, t76896: F, t76899: F, t136: F, t76624: F, t908: F) -> (F, F) {
    let t76901 = t76877 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) * t76880 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t68500 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68502 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t68504 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68506 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t76887 + t76890 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) * t76893 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t76896 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t76899;
    let t76903 = t136 * t908 * t76624;
    (t76901, t76903)
}

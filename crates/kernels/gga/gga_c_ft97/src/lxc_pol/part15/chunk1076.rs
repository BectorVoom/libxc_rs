//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1076/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1076<F: Float>(t605: F, t87113: F, t87128: F, t87144: F, t87160: F, t40530: F, t62364: F, t62410: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F, t86970: F, t86975: F, t86979: F) -> (F, F) {
    let t87163 = t605 * (t87113 + t87128 + t87144 + t87160);
    let t87175 = t62364 + t40530 - F::cast_from(6.0_f64) * t86942 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86946 - F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t86950 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86954 - t86958 / F::cast_from(18.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86962 + t86966 / F::cast_from(3.0_f64) - t86970 / F::cast_from(9.0_f64) + t62410 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86975 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t86979;
    (t87163, t87175)
}

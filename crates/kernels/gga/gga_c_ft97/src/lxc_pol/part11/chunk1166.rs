//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1166/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1166<F: Float>(t44685: F, t44717: F, t43390: F, t43348: F, t43353: F, t43357: F, t43361: F, t43363: F, t43365: F, t43369: F, t43373: F, t43376: F, t43379: F, t43384: F, t43388: F) -> (F, F) {
    let t44718 = t44685 + t44717;
    let t44735 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43390;
    let t44736 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43348 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43353 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43357 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43361 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43363 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43365 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43369 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43373 - F::cast_from(2.0_f64) * t43376 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43379 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43384 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43388 + t44735;
    (t44718, t44736)
}

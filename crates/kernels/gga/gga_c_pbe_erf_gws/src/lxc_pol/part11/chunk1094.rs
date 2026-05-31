//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1094/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1094<F: Float>(t40039: F, t40042: F, t30583: F, t30593: F, t1044: F, t1620: F, t41690: F, t7216: F, t32019: F, t3403: F, t30660: F, t40696: F, t950: F) -> (F, F, F, F, F, F, F, F) {
    let t47545 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40039;
    let t47546 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t40042;
    let t47547 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t30583;
    let t47548 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t30593;
    let t47552 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t1620 * t7216 * t41690 * t1044;
    let t47554 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t32019 * t3403;
    let t47555 = F::cast_from(64.0_f64) / F::cast_from(135.0_f64) * t30660;
    let t47556 = t40696 * t950;
    (t47545, t47546, t47547, t47548, t47552, t47554, t47555, t47556)
}

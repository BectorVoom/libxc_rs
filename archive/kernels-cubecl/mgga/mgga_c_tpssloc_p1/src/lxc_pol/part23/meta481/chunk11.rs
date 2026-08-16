//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1450/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450<F: Float>(t1227: F, t15453: F, t1730: F, t22174: F, t4582: F, t488: F, t6232: F, t65552: F, t65558: F, t65581: F, t65706: F, t72273: F, t72285: F, t72287: F, t72289: F, t72293: F, t72297: F, t72302: F, t77606: F) -> F {
    let t78734 = -F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t1227 * t4582 * t15453 * t77606 + t65552 / F::cast_from(1728.0_f64) + t65706 * t6232 / F::cast_from(48.0_f64) - t72273 / F::cast_from(1728.0_f64) - t65558 / F::cast_from(1152.0_f64) - t72285 / F::cast_from(288.0_f64) + t72287 / F::cast_from(192.0_f64) + t72289 / F::cast_from(108.0_f64) + t72293 / F::cast_from(1152.0_f64) - t72297 / F::cast_from(192.0_f64) - F::cast_from(19.0_f64) / F::cast_from(324.0_f64) * t72302 - F::cast_from(209.0_f64) / F::cast_from(648.0_f64) * t1730 * t22174 * t488 - t65581 / F::cast_from(2304.0_f64);
    t78734
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2073/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2073<F: Float>(t241: F, t6597: F, t248: F, t555: F, t557: F, t12434: F, t1338: F, t12019: F, t566: F, t68: F, t3700: F, t10121: F, t870: F) -> (F, F, F, F, F, F) {
    let t40445 = t6597 * t241;
    let t40449 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t555 * t40445 * t557 * t248;
    let t40479 = t1338 * t12434;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40622 = t10121 * t870;
    (t40445, t40449, t40479, t40591, t40611, t40622)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1226/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1226<F: Float>(t26381: F, t584: F, t5948: F, t7783: F, t5223: F, t7666: F, t21098: F, t4885: F, t959: F, t21224: F, t5249: F, t898: F, t1748: F, t7741: F, t2788: F, t5938: F) -> (F, F, F, F, F, F, F, F) {
    let t26382 = 0.1016176784e-1 * t26381;
    let t26384 = t584 * t7783 * t5948;
    let t26386 = t7666 * t5223;
    let t26388 = 96.0 * t21098;
    let t26389 = t4885 * t959;
    let t26390 = 240.0 * t26389;
    let t26396 = t5249 * t898 * t21224;
    let t26398 = t7741 * t1748;
    let t26399 = 0.21687162600603479684e-1 * t26398;
    let t26400 = t2788 * t5938;
    (t26382, t26384, t26386, t26388, t26390, t26396, t26399, t26400)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1180/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1180<F: Float>(t2314: F, t8326: F, t5113: F, t191: F, t192: F, t6872: F, t2020: F, t6876: F, t8494: F, t6997: F, t8450: F, t1873: F, t23877: F) -> (F, F, F, F, F, F, F) {
    let t31236 = t2314 * t8326;
    let t31237 = F::cast_from(2.0_f64) * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = F::cast_from(2.0_f64) * t31238;
    let t31246 = t6872 * t191 * t192;
    let t31247 = t31246 * t2020;
    let t31249 = t6876 * t8494;
    let t31250 = t8450 * t6997;
    let t31270 = t23877 * t1873;
    (t31237, t31239, t31246, t31247, t31249, t31250, t31270)
}

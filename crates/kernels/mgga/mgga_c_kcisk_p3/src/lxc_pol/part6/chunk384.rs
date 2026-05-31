//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 384/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk384<F: Float>(t1800: F, t2473: F, t1799: F, t2063: F, t682: F, t1819: F, t2366: F, t2373: F, t1815: F, t2372: F, t574: F) -> (F, F, F, F, F) {
    let t2474 = t1800 * t2473;
    let t2475 = t1799 * t2474;
    let t2477 = t682 * t2063;
    let t2484 = F::cast_from(0.1982e-1_f64) * t2373 - t1819 - F::cast_from(0.41275e-2_f64) * t2366;
    let t2487 = t1815 * t2372 / F::cast_from(4.0_f64) + t574 * t2484 / F::cast_from(2.0_f64);
    (t2474, t2475, t2477, t2484, t2487)
}

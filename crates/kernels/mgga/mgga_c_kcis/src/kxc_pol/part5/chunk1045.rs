//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1045/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1045<F: Float>(t1444: F, t1477: F, t1482: F, t1409: F, t3786: F, t1319: F, t1961: F, t1996: F, t3251: F, t3255: F, t5495: F, t5500: F) -> (F, F, F, F, F, F, F) {
    let t16369 = t1477 * t1444;
    let t16373 = t1482 * t1444;
    let t16387 = t3786 * t1409;
    let t16388 = t1961 * t1319;
    let t16401 = t3251 * t1996;
    let t16408 = F::cast_from(0.19711289e-2_f64) * t3255 * t5495;
    let t16410 = F::cast_from(0.26281718666666666666e-2_f64) * t3255 * t5500;
    (t16369, t16373, t16387, t16388, t16401, t16408, t16410)
}

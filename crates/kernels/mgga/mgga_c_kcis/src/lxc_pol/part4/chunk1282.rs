//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1282/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1282<F: Float>(t1444: F, t1961: F, t2642: F, t3766: F, t1996: F, t3251: F, t3815: F, t5804: F, t5498: F, t3255: F, t5495: F, t5500: F) -> (F, F, F, F, F, F) {
    let t16397 = t1961 * t1444 * t2642;
    let t16398 = t3766 * t16397;
    let t16401 = t3251 * t1996;
    let t16403 = t5804 * t3815;
    let t16404 = t5498 * t16403;
    let t16408 = F::new(0.19711289e-2) * t3255 * t5495;
    let t16410 = F::new(0.26281718666666666666e-2) * t3255 * t5500;
    (t16398, t16401, t16403, t16404, t16408, t16410)
}

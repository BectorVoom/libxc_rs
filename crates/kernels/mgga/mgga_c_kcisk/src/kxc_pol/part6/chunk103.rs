//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 103/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk103<F: Float>(t340: F, t379: F, t382: F, rho0: F, sigma0: F) -> (F, F, F, F, F) {
    let t385 = F::new(10.0) / F::new(9.0) * t340 * t379 * t382;
    let t386 = t385 < -F::new(0.66725e-1);
    let t388 = piecewise3::<F>(t386, F::new(0.0), F::new(0.66725e-1) + t385);
    let t389 = t388 * sigma0;
    let t390 = rho0 * rho0;
    let t391 = pow_1_3::<F>(rho0);
    let t392 = t391 * t391;
    let t394 = F::new(1.0) / t392 / t390;
    (t389, t390, t391, t394, t385)
}

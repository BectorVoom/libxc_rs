//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 650/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk650<F: Float>(t577: F, t7321: F, t585: F, t1926: F, t488: F, t579: F, t251: F, t584: F, t578: F, t2061: F, t2065: F, t2038: F, t2042: F) -> (F, F, F, F, F, F, F, F) {
    let t7322 = t7321 * t577;
    let t7323 = t7322 * t585;
    let t7327 = F::new(1.0) / t488 / t579 / t1926;
    let t7328 = t7327 * t251;
    let t7329 = t7328 * t584;
    let t7330 = t578 * t7329;
    let t7332 = t2061 * t2065;
    let t7333 = t578 * t7332;
    let t7335 = t2042 * t2038;
    (t7322, t7323, t7328, t7329, t7330, t7332, t7333, t7335)
}

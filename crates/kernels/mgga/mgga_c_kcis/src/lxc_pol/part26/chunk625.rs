//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 625/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk625<F: Float>(t251: F, t7327: F, t584: F, t578: F, t2061: F, t2065: F, t2038: F, t2042: F, t1533: F, t4261: F, t6917: F, t4260: F, t143: F, t7028: F, t4219: F, t4220: F, t6281: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7328 = t7327 * t251;
    let t7329 = t7328 * t584;
    let t7330 = t578 * t7329;
    let t7332 = t2061 * t2065;
    let t7333 = t578 * t7332;
    let t7335 = t2042 * t2038;
    let t7336 = t1533 * t7335;
    let t7338 = t4261 * t6917;
    let t7339 = t4260 * t7338;
    let t7341 = t7028 * t143;
    let t7361 = t4219 * t4220 * t6281;
    (t7328, t7329, t7330, t7332, t7333, t7335, t7336, t7338, t7339, t7341, t7361)
}

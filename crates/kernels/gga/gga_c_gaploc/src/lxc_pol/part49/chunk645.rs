//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 645/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk645<F: Float>(t10590: F, t188: F, t1589: F, t3358: F, t2482: F, t2890: F, t9267: F, t2299: F, t2875: F, t544: F, t1424: F, t590: F) -> (F, F, F, F, F, F) {
    let t10591 = t188 * t10590;
    let t10594 = t1589 * t3358;
    let t10597 = t2890 * t2482;
    let t10598 = t9267 * t10597;
    let t10599 = F::new(0.9585731488480187419e0) * t10598;
    let t10600 = t2299 * t2875;
    let t10601 = t544 * t10600;
    let t10603 = F::new(0.39722766613167140743e-1) * t10601 * t1424;
    let t10604 = t3358 * t590;
    (t10591, t10594, t10599, t10600, t10603, t10604)
}

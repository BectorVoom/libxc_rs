//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 357/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk357<F: Float>(t1608: F, t2104: F, t286: F, t1597: F, t1599: F, t2096: F, t2100: F, t619: F) -> (F, F) {
    let t2105 = t1608 * t2104;
    let t2106 = t286 * t2105;
    let t2109 = -t2096 * t619 / F::new(72.0) + t1597 + t1599 * t2100 / F::new(576.0) - t1599 * t2106 / F::new(192.0);
    (t2105, t2109)
}

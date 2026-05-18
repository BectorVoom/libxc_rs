//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 307/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk307<F: Float>(t1340: F, t762: F, t531: F, t566: F, t513: F, t516: F, t212: F, t555: F, t225: F, t561: F) -> (F, F, F, F, F, F) {
    let t1342 = F::new(0.5848223622634646207e0) * t1340 * t762;
    let t1343 = t531 * t566;
    let t1344 = F::new(1.0) / t513;
    let t1348 = F::new(1.0) / t516;
    let t1357 = t212 * t555;
    let t1358 = t225 * t561;
    (t1342, t1343, t1344, t1348, t1357, t1358)
}

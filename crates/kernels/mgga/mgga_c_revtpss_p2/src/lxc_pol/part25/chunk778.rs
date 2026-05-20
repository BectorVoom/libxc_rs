//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 778/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk778<F: Float>(t3: F, t7318: F, t1459: F, t2042: F, t116: F, t1936: F, t670: F, t572: F, t117: F, t7002: F, t1461: F, t2040: F, t573: F, param_d: F) -> (F, F, F, F, F, F) {
    let t7319 = t3 * t7318;
    let t7324 = param_d * t7318;
    let t7329 = F::new(3.0) * t1459 * t2042;
    let t7330 = t116 * t1936;
    let t7331 = t7330 * t670;
    let t7333 = F::new(6.0) * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = F::new(3.0) * t572 * t7334;
    let t7337 = F::new(3.0) * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    (t7319, t7324, t7330, t7331, t7334, t7337)
}

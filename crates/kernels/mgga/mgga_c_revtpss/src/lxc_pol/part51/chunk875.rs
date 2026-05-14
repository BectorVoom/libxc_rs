//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 875/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk875<F: Float>(t32300: F, t32341: F, t3: F, t2042: F, t7324: F, t2040: F, t7331: F, t7334: F, t1459: F, t8611: F, t670: F, t8453: F, t572: F, t7002: F, t7330: F, t8614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32342 = t32300 + t32341;
    let t32343 = t3 * t32342;
    let t32354 = param_d * t32342;
    let t32358 = t7324 * t2042;
    let t32360 = t2040 * t7331;
    let t32362 = t2040 * t7334;
    let t32365 = 6.0 * t1459 * t8611;
    let t32366 = t670 * t8453;
    let t32368 = 6.0 * t572 * t32366;
    let t32369 = t7330 * t7002;
    let t32371 = 12.0 * t572 * t32369;
    let t32372 = t1459 * t8614;
    (t32342, t32343, t32354, t32358, t32360, t32362, t32365, t32366, t32368, t32369, t32371, t32372)
}

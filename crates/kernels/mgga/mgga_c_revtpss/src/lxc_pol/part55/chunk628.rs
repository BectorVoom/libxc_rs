//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 628/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk628<F: Float>(t2034: F, t7315: F, t2014: F, t1459: F, t2042: F, t116: F, t1936: F, t670: F, t572: F, t117: F, t7002: F, t38: F, t68: F, t2247: F, t624: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7316 = t2034 * t7315;
    let t7317 = t2014 * t7316;
    let t7329 = 3.0 * t1459 * t2042;
    let t7330 = t116 * t1936;
    let t7331 = t7330 * t670;
    let t7333 = 6.0 * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = 3.0 * t572 * t7334;
    let t7342 = t38 * t68;
    let t7343 = t2247 * t7342;
    let t7348 = t624 * t72;
    (t7316, t7317, t7329, t7330, t7331, t7333, t7334, t7336, t7342, t7343, t7348)
}

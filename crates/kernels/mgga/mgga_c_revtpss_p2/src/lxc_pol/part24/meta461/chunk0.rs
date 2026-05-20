//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1432/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432<F: Float>(t11243: F, t1802: F, t1244: F, t13036: F, t225: F, t56331: F, t480: F, t1235: F, t1789: F, t2434: F, t371: F, t12987: F, t1803: F) -> (F, F, F, F, F, F) {
    let t57403 = t1802 * t11243;
    let t57405 = t13036 * t1244 * t57403;
    let t57465 = t56331 * t225;
    let t57466 = t57465 * t480;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57473 = t12987 * t1803;
    (t57403, t57405, t57465, t57466, t57471, t57473)
}

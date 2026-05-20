//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2621/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2621<F: Float>(t10777: F, t14671: F, t14686: F, t4424: F, t61956: F, t837: F, t18477: F, t50769: F, t51133: F, t18348: F, t2710: F, t2713: F) -> (F, F, F, F) {
    let t62236 = t10777 * t14686 * t14671 * t4424;
    let t62241 = t10777 * t14686 * t61956 * t837;
    let t62246 = t51133 * t50769 * t18477;
    let t62251 = t2710 * t2713 * t18348;
    (t62236, t62241, t62246, t62251)
}

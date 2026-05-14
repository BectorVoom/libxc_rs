//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 819/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk819<F: Float>(t14238: F, t786: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t1904: F, t3895: F) -> (F, F, F, F, F, F, F) {
    let t14239 = t786 * t14238;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    let t14296 = t3895 * t1904;
    (t14239, t14243, t14252, t14280, t14290, t14294, t14296)
}

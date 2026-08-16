//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1371/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1371<F: Float>(t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t1904: F, t3895: F, t2439: F, t213: F, t5710: F) -> (F, F, F, F, F) {
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    let t14296 = t3895 * t1904;
    let t14297 = t2439 * t14296;
    let t14299 = t213 * t5710;
    (t14280, t14290, t14294, t14297, t14299)
}

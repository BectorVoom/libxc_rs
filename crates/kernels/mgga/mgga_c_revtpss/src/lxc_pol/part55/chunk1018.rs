//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1018/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1018<F: Float>(t119967: F, t121245: F, t121204: F, t13847: F, t1399: F, t121210: F, t2453: F, t8705: F, t25304: F, t596: F, t8571: F, t32186: F, t786: F, t119833: F, t121116: F, t32208: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121246 = t119967 * t121245;
    let t121248 = t13847 * t121204 * t1399;
    let t121249 = t121246 * t121248;
    let t121272 = t2453 * t8705 * t121210;
    let t121275 = t25304 * t8705 * t121210;
    let t121305 = t8571 * t596;
    let t121307 = t786 * t121305 * t32186;
    let t121326 = t119833 * t121245;
    let t121327 = t121326 * t121248;
    let t121336 = t121116 * t32208;
    (t121246, t121249, t121272, t121275, t121305, t121307, t121326, t121327, t121336)
}

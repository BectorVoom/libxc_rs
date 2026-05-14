//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 733/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk733<F: Float>(t2516: F, t5571: F, t1320: F, t5569: F, t2626: F, t1856: F, t2608: F, t512: F, t2496: F, t1317: F, t123: F, t2630: F, t1857: F, t3860: F, t3863: F, t1892: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13611 = t5571 * t2516;
    let t13621 = t1320 * t5569;
    let t13630 = t5571 * t2626;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    let t13652 = t5571 * t2496;
    let t13654 = t1317 * t5569;
    let t13665 = t1856 * t123;
    let t13666 = t13665 * t2630;
    let t13668 = t3860 * t1857;
    let t13670 = t3863 * t1857;
    let t13725 = t785 * t1892;
    (t13611, t13621, t13630, t13633, t13652, t13654, t13666, t13668, t13670, t13725)
}

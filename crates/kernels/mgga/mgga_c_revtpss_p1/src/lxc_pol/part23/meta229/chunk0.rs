//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1346/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1346<F: Float>(t467: F, t6594: F, t1785: F, t1803: F, t225: F, t6564: F, t480: F, t482: F, t6573: F, t371: F, t372: F) -> (F, F, F, F, F, F) {
    let t6595 = t467 * t6594;
    let t6598 = t1785 * t1803;
    let t6601 = t6564 * t225;
    let t6602 = t6601 * t480;
    let t6609 = t482 * t6573;
    let t6611 = t371 * t372 * t6609;
    (t6595, t6598, t6601, t6602, t6609, t6611)
}

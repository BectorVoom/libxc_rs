//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 543/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk543<F: Float>(t3992: F, t5609: F, t2661: F, t1413: F, t1868: F, t547: F, t807: F, t221: F, t3979: F, t3978: F, t1885: F, t3930: F, t1856: F, t72: F, t757: F, t539: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5610 = t3992 * t5609;
    let t5611 = t2661 * t5610;
    let t5617 = t1413 * t1868;
    let t5618 = t547 * t5617;
    let t5619 = t807 * t5618;
    let t5622 = t3979 * t221 * t1868;
    let t5623 = t3978 * t5622;
    let t5625 = t3930 * t1885;
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5650 = t539 * t73;
    (t5610, t5611, t5617, t5618, t5619, t5622, t5623, t5625, t5635, t5636, t5650)
}

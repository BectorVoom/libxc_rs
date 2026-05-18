//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 634/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk634<F: Float>(t3992: F, t5609: F, t2661: F, t1414: F, t5591: F, t828: F, t1413: F, t1868: F, t547: F, t807: F, t221: F, t3979: F) -> (F, F, F, F, F) {
    let t5610 = t3992 * t5609;
    let t5611 = t2661 * t5610;
    let t5614 = t1414 * t828 * t5591;
    let t5617 = t1413 * t1868;
    let t5618 = t547 * t5617;
    let t5619 = t807 * t5618;
    let t5622 = t3979 * t221 * t1868;
    (t5611, t5614, t5617, t5619, t5622)
}

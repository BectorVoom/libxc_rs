//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1577/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1577<F: Float>(t1413: F, t22809: F, t547: F, t807: F, t13767: F, t1868: F, t2661: F, t74012: F, t22953: F, t3992: F, t543: F, t550: F) -> (F, F, F) {
    let t86169 = t807 * t547 * t1413 * t22809;
    let t86183 = t2661 * t13767 * t74012 * t1868;
    let t86203 = t2661 * t3992 * t550 * t22953 * t543;
    (t86169, t86183, t86203)
}

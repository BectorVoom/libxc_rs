//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3256/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256<F: Float>(t2661: F, t3992: F, t6869: F, t74026: F, t13999: F, t22843: F, t22854: F, t3989: F, t221: F, t22852: F, t3978: F, t9921: F) -> (F, F, F, F) {
    let t85741 = t2661 * t3992 * t74026 * t6869;
    let t85752 = t13999 * t22843;
    let t85764 = t3989 * t22854;
    let t85776 = t221 * t22852;
    let t85778 = t3978 * t9921 * t85776;
    (t85741, t85752, t85764, t85778)
}

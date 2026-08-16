//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 936/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk936<F: Float>(t225: F, t494: F, t6695: F, t1828: F, t3737: F, t1280: F, t6573: F, t1287: F, t6688: F, t1774: F, t5486: F, t6587: F) -> (F, F, F, F, F, F, F) {
    let t6697 = t6695 * t225 * t494;
    let t6702 = t1828 * t1828;
    let t6703 = t3737 * t6702;
    let t6714 = t1280 * t6573;
    let t6717 = t6688 * t1287;
    let t6720 = t5486 * t1774;
    let t6723 = t1280 * t6587;
    (t6697, t6702, t6703, t6714, t6717, t6720, t6723)
}

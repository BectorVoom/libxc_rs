//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1076/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1076<F: Float>(t12230: F, t6438: F, t3523: F, t6534: F, t12555: F, t6518: F, t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t487: F, t6564: F) -> (F, F, F, F, F, F, F) {
    let t20651 = t6438 * t12230;
    let t20671 = t6534 * t3523;
    let t20678 = t6518 * t12555;
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20753 = t6564 * t487;
    (t20651, t20671, t20678, t20692, t20697, t20700, t20753)
}

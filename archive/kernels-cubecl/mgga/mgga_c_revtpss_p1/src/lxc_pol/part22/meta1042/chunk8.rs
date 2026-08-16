//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3644/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3644<F: Float>(t1131: F, t1150: F, t68821: F, t68837: F, t68854: F, t68870: F, t68887: F, t68903: F, t68920: F, t68936: F, t68779: F, t68781: F, t68784: F, t68786: F, t68789: F, t68791: F, t68794: F, t68799: F, t68803: F, t68805: F, t68808: F) -> (F, F) {
    let t68942 = F::cast_from(1.0_f64) * t1131 * (t68821 + t68837 + t68854 + t68870 + t68887 + t68903 + t68920 + t68936) * t1150;
    let t68943 = t68779 + t68781 + t68784 - t68786 - t68789 + t68791 - t68794 + t68799 + t68803 + t68805 + t68808 + t68942;
    (t68942, t68943)
}

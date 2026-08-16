//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1570/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1570<F: Float>(t125: F, t22857: F, t22809: F, t22953: F, t6843: F, t9994: F, t6869: F, t73731: F, t9816: F, t9818: F, t22829: F, t9962: F) -> (F, F, F, F, F, F) {
    let t85553 = t125 * t22857;
    let t85563 = t125 * t22809;
    let t85609 = t125 * t22953;
    let t85638 = t6843 * t9994;
    let t85648 = t9816 * t9818 * t73731 * t6869;
    let t85652 = t9962 * t22829;
    (t85553, t85563, t85609, t85638, t85648, t85652)
}

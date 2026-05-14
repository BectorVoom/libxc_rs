//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 936/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk936<F: Float>(t25638: F, t31902: F, t25698: F, t7165: F, t247: F, t385: F, t42792: F, t8502: F, t25610: F, t93982: F, t1078: F, t11239: F, t1982: F, t8507: F, t11921: F, t31913: F, t31914: F) -> (F, F, F, F, F, F) {
    let t120584 = t31902 * t25638;
    let t120602 = t25698 * t7165;
    let t120624 = 0.62743259463289926663e-4 * t8502 * t247 * t42792 * t385;
    let t120625 = t25610 * t93982;
    let t120636 = t1982 * t8507 * t11239 * t1078;
    let t120647 = t31913 * t247 * t11921 * t31914;
    (t120584, t120602, t120624, t120625, t120636, t120647)
}

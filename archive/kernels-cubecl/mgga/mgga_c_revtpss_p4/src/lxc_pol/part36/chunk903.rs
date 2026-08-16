//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 903/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk903<F: Float>(t6022: F, t72: F, t686: F, t10530: F, t6017: F, t2798: F, t5978: F, t14568: F, t4500: F, t18699: F, t231: F, t2783: F) -> (F, F, F, F, F) {
    let t18718 = t6022 * t72;
    let t18719 = t18718 * t686;
    let t18720 = t10530 * t18719;
    let t18725 = t6017 * t72;
    let t18726 = t18725 * t686;
    let t18727 = t2798 * t18726;
    let t18729 = t5978 * t72;
    let t18730 = t18729 * t686;
    let t18731 = t2798 * t18730;
    let t18733 = t14568 * t4500;
    let t18738 = t2783 * t18699 * t231;
    (t18720, t18727, t18731, t18733, t18738)
}

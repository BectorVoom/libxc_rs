//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1174/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1174<F: Float>(t6017: F, t72: F, t686: F, t2798: F, t5978: F, t14568: F, t4500: F, t18699: F, t231: F, t2783: F, t2782: F, t18677: F) -> (F, F, F, F, F) {
    let t18725 = t6017 * t72;
    let t18726 = t18725 * t686;
    let t18727 = t2798 * t18726;
    let t18729 = t5978 * t72;
    let t18730 = t18729 * t686;
    let t18731 = t2798 * t18730;
    let t18733 = t14568 * t4500;
    let t18738 = t2783 * t18699 * t231;
    let t18739 = t2782 * t18738;
    let t18742 = t2783 * t18677 * t231;
    (t18727, t18731, t18733, t18739, t18742)
}

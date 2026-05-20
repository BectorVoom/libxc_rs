//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1710/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1710<F: Float>(t3154: F, t999: F, t11659: F, t3117: F, t1086: F, t3046: F, t3090: F) -> (F, F, F, F) {
    let t11860 = t3154 * t999;
    let t11861 = t11659 * t11860;
    let t11862 = t3117 * t11861;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    (t11861, t11862, t11865, t11866)
}

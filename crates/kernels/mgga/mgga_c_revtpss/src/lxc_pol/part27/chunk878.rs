//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 878/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk878<F: Float>(t3129: F, t3172: F, t3127: F, t3135: F, t1041: F, t1065: F, t3059: F, t906: F, t1042: F, t1024: F, t3105: F, t3151: F, t3153: F, t3154: F, t905: F, t606: F) -> (F, F, F, F, F, F) {
    let t11643 = t3172 * t3129;
    let t11644 = t3127 * t11643;
    let t11648 = t3172 * t3135;
    let t11649 = t1041 * t11648;
    let t11651 = t1065 * t3059;
    let t11652 = t11651 * t906;
    let t11653 = t1042 * t11652;
    let t11656 = t1024 * t3105;
    let t11659 = t3151 * t3153;
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    (t11644, t11649, t11653, t11656, t11659, t11661)
}

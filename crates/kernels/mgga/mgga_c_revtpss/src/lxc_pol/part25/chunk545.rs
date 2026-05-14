//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 545/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk545<F: Float>(t1032: F, t989: F, t1040: F, t1024: F, t1062: F, t1065: F, t999: F, t906: F, t1042: F) -> (F, F, F, F, F, F) {
    let t3123 = t989 * t1032;
    let t3124 = t3123 * t1040;
    let t3127 = t1024 * t1062;
    let t3128 = t1065 * t999;
    let t3129 = t3128 * t906;
    let t3130 = t1042 * t3129;
    (t3123, t3124, t3127, t3128, t3129, t3130)
}

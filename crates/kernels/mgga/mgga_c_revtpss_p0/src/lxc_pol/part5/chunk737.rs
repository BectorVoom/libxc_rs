//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 737/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk737<F: Float>(t342: F, t4980: F, t3154: F, t3302: F, t1043: F, t4893: F, t1071: F, t1089: F, t1668: F, t378: F, t4866: F, t3316: F) -> (F, F, F, F, F, F, F) {
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4983 = t4982 * t1043;
    let t4984 = t4893 * t4983;
    let t4988 = t1071 * t1668 * t1089;
    let t4992 = t378 * t4866 * t1089;
    let t4995 = t3316 * t378;
    (t4981, t4982, t4983, t4984, t4988, t4992, t4995)
}

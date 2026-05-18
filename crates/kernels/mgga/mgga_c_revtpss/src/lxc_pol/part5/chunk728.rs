//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 728/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk728<F: Float>(t1015: F, t4186: F, t1012: F, t3147: F, t72: F, t3088: F) -> (F, F, F, F) {
    let t4886 = t1015 * t4186;
    let t4887 = t1012 * t4886;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    (t4886, t4887, t4890, t4891)
}

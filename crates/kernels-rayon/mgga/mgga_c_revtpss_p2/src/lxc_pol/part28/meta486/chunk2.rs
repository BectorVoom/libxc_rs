//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1849/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1849(t1312: f64, t25832: f64, t2371: f64, t25096: f64, t25169: f64, t25805: f64, t25812: f64, t25814: f64, t25816: f64, t25818: f64, t25820: f64, t670: f64, t6985: f64) -> f64 {
    let t25834 = 2.0_f64 * t1312 * t25832;
    let t25835 = 2.0_f64 * t2371 * t6985 + 4.0_f64 * t25805 * t670 + 2.0_f64 * t25096 + t25169 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834;
    t25835
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1083/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1083(t2096: f64, t2105: f64, t254: f64, t265: f64, t6079: f64, t10868: f64, t277: f64) -> (f64, f64) {
    let t38143 = t254 * t6079 * t2096 * t265 * t2105;
    let t38144 = 0.11579802508189808742e1_f64 * t38143;
    let t38145 = t10868 * t277;
    (t38144, t38145)
}

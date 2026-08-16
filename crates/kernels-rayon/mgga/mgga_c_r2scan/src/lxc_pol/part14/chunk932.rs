//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 932/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk932(t2096: f64, t2101: f64, t2105: f64, t265: f64, t254: f64, t277: f64, t3332: f64) -> (f64, f64, f64) {
    let t10757 = t2101 * t2096 * t265 * t2105;
    let t10758 = t254 * t10757;
    let t10760 = t3332 * t277;
    (t10757, t10758, t10760)
}

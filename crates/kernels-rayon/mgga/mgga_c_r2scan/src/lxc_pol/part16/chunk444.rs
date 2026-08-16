//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 444/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk444(t2168: f64, t565: f64, t110: f64, t1598: f64, t524: f64, t531: f64, t108: f64, t144: f64, t543: f64) -> (f64, f64, f64, f64, f64) {
    let t2169 = t565 * t2168;
    let t2176 = t1598 * t110;
    let t2177 = t524 * t2176;
    let t2178 = t2177 * t531;
    let t2182 = t108 / t543 / t144;
    (t2169, t2176, t2177, t2178, t2182)
}

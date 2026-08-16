//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 902/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk902(t6953: f64, t6964: f64, t7013: f64, t7022: f64, t7053: f64, t7113: f64, t7139: f64, t8303: f64, t6621: f64, t990: f64, t1249: f64, t1248: f64, t295: f64) -> (f64, f64, f64) {
    let t8306 = t6953 + t6964 + t7013 + t7022 + t7053 + t7113 + t7139 + t8303;
    let t8315 = t6621 * t990;
    let t8316 = t8315 * t1249;
    let t8319 = t295 * t1248;
    (t8306, t8316, t8319)
}

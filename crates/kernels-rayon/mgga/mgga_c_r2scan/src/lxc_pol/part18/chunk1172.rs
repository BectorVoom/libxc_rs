//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1172/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1172(t10781: f64, t9513: f64, t3308: f64, t574: f64, t9147: f64, t1054: f64, t2139: f64, t8752: f64, t2133: f64, t8736: f64, t40194: f64, t40195: f64, t8756: f64) -> (f64, f64, f64, f64, f64) {
    let t43009 = t10781 * t9513;
    let t43012 = t574 * t3308 * t9147;
    let t43015 = t2139 * t1054 * t8752;
    let t43018 = t2133 * t1054 * t8736;
    let t43021 = t40194 * t40195 * t8756;
    (t43009, t43012, t43015, t43018, t43021)
}

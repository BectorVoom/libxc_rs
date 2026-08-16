//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 314/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk314(t1058: f64, t1060: f64, t783: f64, t1051: f64, t1056: f64) -> f64 {
    let t1062 = t783 * t1058 * t1060;
    let t1064 = 0.27439371595564631661e-1_f64 * t1051 + 0.43341108700271342816e-1_f64 * t1056 - 0.21831846657716620896e-2_f64 * t1062;
    t1064
}

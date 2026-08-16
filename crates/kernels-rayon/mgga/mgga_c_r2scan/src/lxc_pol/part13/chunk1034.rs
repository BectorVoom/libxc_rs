//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1034/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1034(t537: f64, t7194: f64, t113: f64, t24165: f64, t24118: f64, t2185: f64, t921: f64, t19790: f64, t1553: f64, t7338: f64, t2654: f64, t6212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25191 = t537 * t7194;
    let t25192 = t25191 * t113;
    let t25303 = t24165 * t113;
    let t25307 = t24118 * t113;
    let t25314 = t921 * t2185;
    let t25397 = t19790 * t921;
    let t25466 = t7338 * t1553;
    let t25480 = t6212 * t2654;
    (t25192, t25303, t25307, t25314, t25397, t25466, t25480)
}

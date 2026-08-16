//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1118/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1118(t1060: f64, t269: f64, t783: f64, t7916: f64, t24906: f64, t37943: f64, t37945: f64, t24916: f64, t37949: f64, t2184: f64, t25746: f64, t3308: f64) -> (f64, f64, f64, f64) {
    let t39476 = t783 * t7916 * t269 * t1060;
    let t39482 = t37943 * t37945 * t24906;
    let t39485 = t37949 * t37945 * t24916;
    let t39490 = t2184 * t3308 * t25746;
    (t39476, t39482, t39485, t39490)
}

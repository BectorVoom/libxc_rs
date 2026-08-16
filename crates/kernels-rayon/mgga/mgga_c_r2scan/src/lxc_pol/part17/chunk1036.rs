//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1036/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1036(t2096: f64, t128: f64, t4145: f64, t133: f64, t5052: f64, t10878: f64, t545: f64, t20094: f64, t2182: f64, t3303: f64, t146: f64, t6533: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20544 = t2096 * t2096;
    let t20621 = t4145 * t128;
    let t20946 = t5052 * t133;
    let t22731 = t545 * t10878;
    let t22766 = t20094 * t128;
    let t22790 = t2182 * t3303;
    let t22796 = t146 * t6533 * t774;
    (t20544, t20621, t20946, t22731, t22766, t22790, t22796)
}

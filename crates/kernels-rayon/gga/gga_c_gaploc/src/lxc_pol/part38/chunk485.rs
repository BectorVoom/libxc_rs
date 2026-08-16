//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 485/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk485(t1359: f64, t986: f64, t107: f64, t7887: f64, t544: f64, t2760: f64, t1339: f64, t2754: f64, t1: f64, t8025: f64, t1415: f64, t2967: f64, t747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8237 = t1359 * t986;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    let t8261 = t2760 * t107;
    let t8272 = t1339 * t2754;
    let t8330 = t8025 * t1;
    let t8331 = t544 * t8330;
    let t8410 = t7887 * t1;
    let t8411 = t1415 * t8410;
    let t8440 = t2967 * t747;
    (t8237, t8247, t8248, t8261, t8272, t8331, t8410, t8411, t8440)
}

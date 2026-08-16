//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1145/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1145(t29273: f64, t7923: f64, t4153: f64, t27387: f64, t7100: f64, t1394: f64, t6904: f64, t1889: f64, t5885: f64, t5709: f64, t1943: f64, t28342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29274 = t7923 * t29273;
    let t29275 = t4153 * t29274;
    let t29277 = t27387 * t7100;
    let t29278 = t1394 * t29277;
    let t29280 = t7923 * t6904;
    let t29281 = t1394 * t29280;
    let t29283 = t5885 * t1889;
    let t29284 = t5709 * t29283;
    let t29288 = t28342 * t1943;
    (t29274, t29275, t29277, t29278, t29280, t29281, t29283, t29284, t29288)
}

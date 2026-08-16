//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 765/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk765(t166: f64, t6880: f64, t2320: f64, t58: f64, t766: f64, t2332: f64, t287: f64, t4881: f64, t4886: f64, t4896: f64, t2850: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6881 = t6880 * t166;
    let t6887 = t2320 * t58;
    let t6888 = t6887 * t766;
    let t6897 = 1.0_f64 / t2332 / t287;
    let t6946 = 12.0_f64 * t4881;
    let t6948 = 80.0_f64 * t4886;
    let t6951 = 32.0_f64 * t4896;
    let t6955 = t2850 * t797;
    (t6881, t6887, t6888, t6897, t6946, t6948, t6951, t6955)
}

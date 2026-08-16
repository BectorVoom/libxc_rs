//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 961/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk961(t10971: f64, t3448: f64, t10648: f64, t10933: f64, t10937: f64, t10942: f64, t10945: f64, t10948: f64, t10952: f64, t10957: f64, t10960: f64, t10965: f64, t10970: f64) -> (f64, f64, f64) {
    let t10972 = t10971 * t3448;
    let t10973 = t10648 * t10972;
    let t10974 = 0.30487649791575028314e-3_f64 * t10973;
    let t10975 = -t10933 + 0.19211284388664477842e-2_f64 * t10937 - t10942 + t10945 + t10948 + 0.43368970657079495312e-4_f64 * t10952 + t10957 - 0.30487649791575028314e-3_f64 * t10960 - t10965 + t10970 + t10974;
    (t10972, t10974, t10975)
}

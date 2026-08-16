//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1426/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1426(t35141: f64, t26822: f64, t901: f64, t10315: f64, t20445: f64, t12963: f64, t1540: f64, t31347: f64, t31358: f64, t31361: f64, t35120: f64, t35123: f64, t35126: f64, t35128: f64, t35130: f64, t35133: f64, t35136: f64, t35138: f64, t35140: f64, t4130: f64, t4781: f64) -> f64 {
    let t35142 = 0.29792074959875355558e-1_f64 * t35141;
    let t35143 = t26822 * t901;
    let t35144 = 0.14896037479937677779e-1_f64 * t35143;
    let t35146 = 0.14300195980740170668e1_f64 * t20445 * t10315;
    let t35151 = -t35120 - t31347 - t35123 + t35126 + t35128 - t35130 - t35133 + t35136 - t35138 + t35140 + t35142 + t35144 - t35146 + 0.30674340763136599742e1_f64 * t4781 * t4130 * t12963 * t1540 - t31358 - t31361;
    t35151
}

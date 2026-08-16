//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1158/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1158(t11708: f64, t6425: f64, t10836: f64, t7601: f64, t2147: f64, t2608: f64, t38168: f64, t10855: f64, t128: f64, t512: f64, t7625: f64, t10760: f64, t24172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39874 = t6425 * t11708;
    let t39879 = t7601 * t10836;
    let t39882 = t2147 * t38168 * t2608;
    let t39885 = t512 * t10855 * t128;
    let t39886 = t39885 * t7625;
    let t39887 = 0.97574405393827830186e-2_f64 * t39886;
    let t39891 = t2147 * t10760 * t24172;
    (t39874, t39879, t39882, t39885, t39887, t39891)
}

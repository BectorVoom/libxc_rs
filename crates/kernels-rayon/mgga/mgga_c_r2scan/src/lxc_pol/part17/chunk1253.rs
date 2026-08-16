//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1253/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1253(t1013: f64, t11223: f64, t12241: f64, t12838: f64, t12841: f64, t12844: f64, t1300: f64, t19203: f64, t2394: f64, t2400: f64, t2944: f64, t3506: f64, t3730: f64, t3735: f64, t38783: f64, t41906: f64, t6693: f64, t829: f64, t9687: f64, t9690: f64, t9693: f64) -> f64 {
    let t44609 = -0.768e1_f64 * t6693 * t3735 * t2394 - 0.1536e2_f64 * t19203 * t12838 * t829 - 0.768e1_f64 * t6693 * t12841 * t829 - 0.384e1_f64 * t6693 * t12844 * t829 - 0.768e1_f64 * t41906 * t2400 - 0.768e1_f64 * t11223 * t9690 - 0.1536e2_f64 * t38783 * t9687 - 0.384e1_f64 * t11223 * t9693 - 0.384e1_f64 * t6693 * t3506 * t2944 - 0.256e1_f64 * t1300 * t12241 * t1013 - 0.256e1_f64 * t1300 * t3730 * t2394;
    t44609
}

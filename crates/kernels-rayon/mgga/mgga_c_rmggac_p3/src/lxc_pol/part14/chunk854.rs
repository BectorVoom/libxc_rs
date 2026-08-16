//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 854/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk854(t1182: f64, t551: f64, t1970: f64, t209: f64, t236: f64, t3352: f64, t7244: f64, t9159: f64, t1971: f64, t3351: f64, t5156: f64, t7190: f64) -> (f64, f64, f64, f64) {
    let t38928 = t551 * t1182;
    let t38932 = t1970 * t3352 * t236 * t38928 * t209;
    let t38934 = t7244 * t9159;
    let t38938 = t3351 * t1971 * t7190 * t5156;
    (t38928, t38932, t38934, t38938)
}

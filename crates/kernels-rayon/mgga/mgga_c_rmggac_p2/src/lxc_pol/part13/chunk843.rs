//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 843/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk843(t2144: f64, t3351: f64, t3352: f64, t5145: f64, t1971: f64, t5268: f64, t7262: f64, t1182: f64, t551: f64, t1970: f64, t209: f64, t236: f64) -> (f64, f64, f64, f64) {
    let t38922 = t3351 * t3352 * t2144 * t5145;
    let t38926 = t3351 * t1971 * t7262 * t5268;
    let t38928 = t551 * t1182;
    let t38932 = t1970 * t3352 * t236 * t38928 * t209;
    (t38922, t38926, t38928, t38932)
}

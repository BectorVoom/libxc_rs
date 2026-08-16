//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 815/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk815(t2131: f64, t5026: f64, t7244: f64, t9171: f64, t1540: f64, t2144: f64, t36734: f64, t8443: f64, t8437: f64, t36292: f64, t5888: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39923 = t5026 * t2131;
    let t39926 = t7244 * t9171;
    let t39927 = 0.19863479950205658386e-4_f64 * t39926;
    let t39953 = t1540 * t2144;
    let t39970 = t36734 * t8443;
    let t39971 = 0.19863479950205658386e-4_f64 * t39970;
    let t39977 = t7244 * t8437;
    let t39978 = 0.19863479950205658386e-4_f64 * t39977;
    let t39997 = t739 * t36292 * t5888;
    (t39923, t39927, t39953, t39971, t39978, t39997)
}

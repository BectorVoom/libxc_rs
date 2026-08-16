//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 924/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk924(t2318: f64, t34975: f64, t35039: f64, t8420: f64, t16504: f64, t8425: f64, t3369: f64, t8430: f64, t34976: f64, t39866: f64, t8435: f64, t321: f64, t9876: f64) -> (f64, f64, f64, f64, f64) {
    let t45403 = t34975 * t35039 * t2318 * t8420;
    let t45407 = t34975 * t16504 * t2318 * t8425;
    let t45411 = t34975 * t3369 * t2318 * t8430;
    let t45415 = t34975 * t34976 * t39866 * t8435;
    let t45418 = t9876 * t321;
    (t45403, t45407, t45411, t45415, t45418)
}

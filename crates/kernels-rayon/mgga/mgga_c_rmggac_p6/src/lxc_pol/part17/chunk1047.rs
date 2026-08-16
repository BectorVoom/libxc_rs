//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1047/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1047(t40731: f64, t8571: f64, t1981: f64, t632: f64, t676: f64, t8512: f64, t39300: f64, t40246: f64, t1971: f64, t236: f64, t35331: f64, t6135: f64) -> (f64, f64, f64, f64) {
    let t47219 = t8571 * t40731;
    let t47223 = t8512 * t1981 * t676 * t632;
    let t47225 = t39300 * t40246;
    let t47229 = t35331 * t1971 * t236 * t6135;
    (t47219, t47223, t47225, t47229)
}

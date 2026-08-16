//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 785/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk785(t14124: f64, t21714: f64, t236: f64, t321: f64, t598: f64, t14125: f64, t333: f64, t511: f64, t352: f64, t515: f64, t15367: f64, t69568: f64) -> (f64, f64, f64, f64) {
    let t74142 = t14124 * t21714 * t236 * t598 * t321;
    let t74147 = t14124 * t14125 * t511 * t598 * t333;
    let t74152 = t14124 * t14125 * t515 * t598 * t352;
    let t74154 = t69568 * t15367;
    (t74142, t74147, t74152, t74154)
}

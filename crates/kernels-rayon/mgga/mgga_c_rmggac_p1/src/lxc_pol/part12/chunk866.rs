//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 866/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk866(t1462: f64, t236: f64, t321: f64, t3352: f64, t8517: f64, t1243: f64, t1475: f64, t1970: f64, t7231: f64, t833: f64, t333: f64, t511: f64, t7230: f64, t8829: f64) -> (f64, f64, f64, f64) {
    let t39079 = t8517 * t3352 * t236 * t1462 * t321;
    let t39084 = t1970 * t7231 * t236 * t1475 * t1243;
    let t39089 = t1970 * t3352 * t236 * t1475 * t833;
    let t39094 = t7230 * t3352 * t511 * t8829 * t333;
    (t39079, t39084, t39089, t39094)
}

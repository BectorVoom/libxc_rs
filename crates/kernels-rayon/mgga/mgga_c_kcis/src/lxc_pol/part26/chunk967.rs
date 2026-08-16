//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 967/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk967(t1396: f64, t22252: f64, t4123: f64, t1464: f64, t15955: f64, t5671: f64, t12241: f64, t15909: f64, t3728: f64, t6919: f64, t17292: f64, t5663: f64) -> (f64, f64, f64, f64, f64) {
    let t22253 = t1396 * t22252;
    let t22254 = t4123 * t22253;
    let t22255 = t1464 * t22254;
    let t22259 = t15955 * t5671;
    let t22260 = t12241 * t22259;
    let t22261 = t15909 * t22260;
    let t22263 = t3728 * t6919;
    let t22265 = t17292 * t5663;
    (t22255, t22259, t22261, t22263, t22265)
}

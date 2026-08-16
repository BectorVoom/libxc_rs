//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 824/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk824(t11881: f64, t1948: f64, t4142: f64, t5773: f64, t1495: f64, t4169: f64, t1396: f64, t4161: f64, t12240: f64, t5770: f64, t1017: f64, t541: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15826 = t11881 * t1948;
    let t15844 = t4142 * t5773;
    let t15865 = t4169 * t1495;
    let t15878 = t4161 * t1396;
    let t15887 = t12240 * t1396;
    let t15896 = t4142 * t5770;
    let t15909 = t86 * t1017 * t541;
    (t15826, t15844, t15865, t15878, t15887, t15896, t15909)
}

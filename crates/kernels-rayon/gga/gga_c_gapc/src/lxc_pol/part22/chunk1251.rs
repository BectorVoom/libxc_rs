//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1251/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1251(t1040: f64, t34681: f64, t1030: f64, t26396: f64, t34058: f64, t11546: f64, t424: f64, t641: f64, t655: f64, t1266: f64, t3696: f64, t3703: f64) -> (f64, f64, f64, f64, f64) {
    let t34891 = t34681 * t1040;
    let t34894 = t1030 * t34058 * t26396;
    let t34897 = t424 * t641 * t11546;
    let t34900 = t424 * t655 * t11546;
    let t34905 = t1266 * t3696 * t3703;
    (t34891, t34894, t34897, t34900, t34905)
}

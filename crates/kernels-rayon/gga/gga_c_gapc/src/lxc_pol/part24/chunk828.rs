//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 828/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk828(t1034: f64, t9906: f64, t1089: f64, t3364: f64, t3368: f64, t103: f64, t2188: f64, t1088: f64, t1085: f64, t3072: f64, t3363: f64, t3322: f64) -> (f64, f64, f64, f64, f64) {
    let t9907 = t9906 * t1034;
    let t9908 = t9907 * t1089;
    let t9910 = t3364 * t3368;
    let t9912 = t103 * t2188;
    let t9913 = t9912 * t1088;
    let t9914 = t1085 * t9913;
    let t9916 = t3363 * t3072;
    let t9917 = t9916 * t3322;
    (t9908, t9910, t9913, t9914, t9917)
}

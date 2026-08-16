//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 870/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk870(t3728: f64, t7263: f64, t7204: f64, t2001: f64, t4134: f64, t5875: f64, t4162: f64, t15909: f64, t17298: f64, t5645: f64, t5650: f64, t3717: f64, t7091: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20900 = t3728 * t7263;
    let t20902 = t3728 * t7204;
    let t20905 = t4134 * t2001;
    let t20906 = t20905 * t5875;
    let t20907 = t4162 * t20906;
    let t20908 = t15909 * t20907;
    let t20910 = t17298 * t5645;
    let t20912 = t17298 * t5650;
    let t20916 = t7091 * t3717;
    (t20900, t20902, t20906, t20908, t20910, t20912, t20916)
}

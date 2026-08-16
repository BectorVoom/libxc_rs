//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2945/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945(t1043: f64, t1469: f64, t3133: f64, t3162: f64, t3115: f64, t42793: f64, t4906: f64, t1045: f64, t11774: f64, t15584: f64, t15689: f64, t15691: f64, t16226: f64, t16227: f64, t19980: f64, t2251: f64, t2258: f64, t2852: f64, t3075: f64, t3155: f64, t42324: f64, t42326: f64, t42334: f64, t42336: f64, t42338: f64, t43301: f64, t606: f64, t905: f64) -> (f64, f64) {
    let t53585 = t1469 * t1043;
    let t53586 = t3162 * t3133;
    let t53612 = t3115 * t42793 * t4906;
    let t53613 = 0.14291339372689912324e-3_f64 * t53612;
    let t53617 = 0.15244095330869239812e-2_f64 * t42324 + 0.19055119163586549765e-3_f64 * t42326 - 0.42874018118069736972e-3_f64 * t15689 * t15584 * t53585 * t53586 - 0.42874018118069736972e-3_f64 * t11774 * t15691 * t1045 * t3075 * t905 * t606 - 0.42874018118069736972e-3_f64 * t11774 * t15691 * t43301 + 0.85748036236139473944e-3_f64 * t16226 * t15691 * t3155 * t16227 * t2258 + 0.14291339372689912324e-2_f64 * t16226 * t19980 * t3155 * t1043 * t2852 * t2251 + t53613 - 0.30488190661738479624e-2_f64 * t42334 - 0.15244095330869239812e-2_f64 * t42336 - 0.2540682555144873302e-2_f64 * t42338;
    (t53585, t53617)
}

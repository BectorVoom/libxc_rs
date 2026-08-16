//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2945/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2945<F: Float>(t1043: F, t1469: F, t3133: F, t3162: F, t3115: F, t42793: F, t4906: F, t1045: F, t11774: F, t15584: F, t15689: F, t15691: F, t16226: F, t16227: F, t19980: F, t2251: F, t2258: F, t2852: F, t3075: F, t3155: F, t42324: F, t42326: F, t42334: F, t42336: F, t42338: F, t43301: F, t606: F, t905: F) -> (F, F) {
    let t53585 = t1469 * t1043;
    let t53586 = t3162 * t3133;
    let t53612 = t3115 * t42793 * t4906;
    let t53613 = F::cast_from(0.14291339372689912324e-3_f64) * t53612;
    let t53617 = F::cast_from(0.15244095330869239812e-2_f64) * t42324 + F::cast_from(0.19055119163586549765e-3_f64) * t42326 - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t15584 * t53585 * t53586 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15691 * t1045 * t3075 * t905 * t606 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15691 * t43301 + F::cast_from(0.85748036236139473944e-3_f64) * t16226 * t15691 * t3155 * t16227 * t2258 + F::cast_from(0.14291339372689912324e-2_f64) * t16226 * t19980 * t3155 * t1043 * t2852 * t2251 + t53613 - F::cast_from(0.30488190661738479624e-2_f64) * t42334 - F::cast_from(0.15244095330869239812e-2_f64) * t42336 - F::cast_from(0.2540682555144873302e-2_f64) * t42338;
    (t53585, t53617)
}

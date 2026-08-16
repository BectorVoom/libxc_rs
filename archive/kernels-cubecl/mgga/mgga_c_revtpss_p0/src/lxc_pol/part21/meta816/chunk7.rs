//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3002/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3002<F: Float>(t12047: F, t53740: F, t16138: F, t372: F, t16158: F, t3106: F, t12003: F, t1659: F, t11648: F, t4879: F, t1042: F, t1068: F, t11286: F, t11679: F, t11705: F, t11774: F, t11983: F, t11994: F, t12131: F, t15689: F, t15691: F, t15696: F, t15697: F, t15707: F, t16140: F, t19980: F, t3075: F, t3095: F, t3096: F, t3127: F, t4186: F, t42155: F, t42804: F, t4834: F, t4872: F) -> F {
    let t54811 = t12047 * t53740;
    let t54818 = t372 * t16138;
    let t54836 = t3106 * t16158;
    let t54838 = t1659 * t12003;
    let t54841 = t4879 * t11648;
    let t54843 = -F::cast_from(0.7145669686344956162e-3_f64) * t15689 * t19980 * t12131 * t11705 + F::cast_from(0.42874018118069736972e-3_f64) * t54811 * t15691 * t42804 * t3095 - F::cast_from(0.85748036236139473944e-3_f64) * t42155 * t15697 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t54818 * t3096 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15696 * t11679 - F::cast_from(0.7145669686344956162e-3_f64) * t15707 * t11286 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t4872 * t4186 * t3075 - F::cast_from(0.85748036236139473944e-3_f64) * t11994 * t16140 + F::cast_from(0.71456696863449561621e-3_f64) * t4834 * t11983 - F::cast_from(0.30488190661738479624e-2_f64) * t54836 + F::cast_from(0.14481890564325777821e-1_f64) * t54838 * t1068 + F::cast_from(0.42874018118069736972e-3_f64) * t54841;
    t54843
}

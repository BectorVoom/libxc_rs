//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3543/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3543(t11859: f64, t11922: f64, t20074: f64, t15926: f64, t16035: f64, t1045: f64, t11672: f64, t11774: f64, t12004: f64, t15154: f64, t15600: f64, t15696: f64, t15700: f64, t15899: f64, t19980: f64, t20079: f64, t43238: f64, t43242: f64, t4787: f64, t54570: f64, t54818: f64, t55152: f64, t55154: f64, t55171: f64, t6323: f64) -> f64 {
    let t67327 = t11859 * t11922 * t20074;
    let t67329 = t15926 * t16035;
    let t67345 = 0.30488190661738479624e-2_f64 * t55152 + 0.19055119163586549765e-3_f64 * t55154 - 0.67751534803863288054e-3_f64 * t43238 + 0.48272968547752592739e-2_f64 * t12004 * t6323 + 0.42874018118069736972e-3_f64 * t54570 * t15899 - 0.57165357490759649296e-3_f64 * t67327 - 0.57165357490759649296e-3_f64 * t67329 - 0.57165357490759649296e-3_f64 * t55171 - 0.6351706387862183255e-4_f64 * t43242 - 0.15244095330869239812e-2_f64 * t11672 * t20079 - 0.28582678745379824648e-2_f64 * t15700 * t19980 * t1045 * t15154 - 0.57165357490759649296e-3_f64 * t11774 * t54818 * t4787 - 0.28582678745379824648e-3_f64 * t11774 * t15696 * t15600;
    t67345
}

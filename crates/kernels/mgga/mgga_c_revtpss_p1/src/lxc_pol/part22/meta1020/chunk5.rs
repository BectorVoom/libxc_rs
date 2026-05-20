//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3543/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3543<F: Float>(t11859: F, t11922: F, t20074: F, t15926: F, t16035: F, t1045: F, t11672: F, t11774: F, t12004: F, t15154: F, t15600: F, t15696: F, t15700: F, t15899: F, t19980: F, t20079: F, t43238: F, t43242: F, t4787: F, t54570: F, t54818: F, t55152: F, t55154: F, t55171: F, t6323: F) -> F {
    let t67327 = t11859 * t11922 * t20074;
    let t67329 = t15926 * t16035;
    let t67345 = F::cast_from(0.30488190661738479624e-2_f64) * t55152 + F::cast_from(0.19055119163586549765e-3_f64) * t55154 - F::cast_from(0.67751534803863288054e-3_f64) * t43238 + F::cast_from(0.48272968547752592739e-2_f64) * t12004 * t6323 + F::cast_from(0.42874018118069736972e-3_f64) * t54570 * t15899 - F::cast_from(0.57165357490759649296e-3_f64) * t67327 - F::cast_from(0.57165357490759649296e-3_f64) * t67329 - F::cast_from(0.57165357490759649296e-3_f64) * t55171 - F::cast_from(0.6351706387862183255e-4_f64) * t43242 - F::cast_from(0.15244095330869239812e-2_f64) * t11672 * t20079 - F::cast_from(0.28582678745379824648e-2_f64) * t15700 * t19980 * t1045 * t15154 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t54818 * t4787 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15696 * t15600;
    t67345
}

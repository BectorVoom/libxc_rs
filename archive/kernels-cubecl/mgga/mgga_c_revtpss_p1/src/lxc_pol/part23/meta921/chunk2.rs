//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2973/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2973<F: Float>(t4772: F, t6244: F, t1041: F, t1042: F, t1045: F, t1062: F, t11656: F, t11703: F, t15716: F, t15728: F, t16089: F, t19707: F, t23643: F, t23823: F, t23859: F, t23966: F, t247: F, t2852: F, t3116: F, t3124: F, t373: F, t4181: F, t42879: F, t42914: F, t4839: F, t55202: F, t6308: F, t65347: F, t65357: F, t65359: F, t65376: F, t65431: F, t65444: F, t65446: F, t66047: F, t67501: F, t78676: F, t78721: F) -> (F, F) {
    let t78740 = t6244 * t4772;
    let t78745 = F::cast_from(0.14291339372689912324e-2_f64) * t16089 * t11703 * t6244 * t2852 * t4181 + F::cast_from(0.17149607247227894789e-2_f64) * t66047 * t19707 - F::cast_from(0.30488190661738479624e-2_f64) * t65347 - F::cast_from(0.95275595817932748827e-4_f64) * t65357 - F::cast_from(0.15244095330869239812e-2_f64) * t65359 - F::cast_from(0.11433071498151929859e-2_f64) * t42879 * t23643 + F::cast_from(0.14291339372689912324e-3_f64) * t78676 + F::cast_from(0.22866142996303859718e-2_f64) * t11656 * t23859 + F::cast_from(0.21437009059034868486e-3_f64) * t3124 * t23823 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1042 * t373 * t78721 * t1045 + F::cast_from(0.12862205435420921092e-2_f64) * t55202 * t6308 + F::cast_from(0.21437009059034868486e-3_f64) * t42914 * t23643 - F::cast_from(0.28582678745379824648e-3_f64) * t65376 - F::cast_from(0.22866142996303859718e-2_f64) * t65431 + F::cast_from(0.57165357490759649295e-3_f64) * t65444 - F::cast_from(0.57165357490759649296e-3_f64) * t65446 + F::cast_from(0.12862205435420921092e-2_f64) * t67501 * t1062 * t4839 - F::cast_from(0.68598428988911579157e-2_f64) * t15728 * t23966 - F::cast_from(0.38586616306262763276e-2_f64) * t15716 * t247 * t3116 * t78740;
    (t78740, t78745)
}

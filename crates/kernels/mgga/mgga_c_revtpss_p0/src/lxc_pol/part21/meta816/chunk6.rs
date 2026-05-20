//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3001/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3001<F: Float>(t11710: F, t15591: F, t3091: F, t16060: F, t3241: F, t1011: F, t140: F, t16122: F, t12078: F, t53740: F, t11661: F, t11684: F, t11696: F, t11883: F, t11927: F, t12131: F, t15618: F, t15689: F, t15691: F, t15700: F, t15717: F, t15957: F, t16025: F, t16190: F, t19980: F, t3117: F, t3136: F, t42316: F, t42804: F, t43291: F, t4786: F, t4887: F) -> F {
    let t54785 = t3091 * t11710 * t15591;
    let t54792 = t3241 * t16060;
    let t54795 = t1011 * t140 * t16122;
    let t54801 = t12078 * t53740;
    let t54806 = -F::cast_from(0.34299214494455789577e-2_f64) * t16190 * t3136 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t15957 * t16025 - F::cast_from(0.85748036236139473944e-3_f64) * t15618 * t11684 - F::cast_from(0.38586616306262763275e-2_f64) * t43291 * t3117 * t15717 * t4786 + F::cast_from(0.28582678745379824648e-3_f64) * t54785 + F::cast_from(0.71456696863449561621e-3_f64) * t15700 * t19980 * t42316 + F::new(11.0) / F::new(108.0) * t11883 * t4887 - t54792 / F::new(54.0) + t54795 / F::new(288.0) - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t15691 * t12131 * t11696 - F::cast_from(0.25724410870841842183e-2_f64) * t54801 * t15691 * t42804 * t11661;
    t54806
}

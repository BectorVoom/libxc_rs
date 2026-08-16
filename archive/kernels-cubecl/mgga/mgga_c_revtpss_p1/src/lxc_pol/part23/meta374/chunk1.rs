//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1706/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1706<F: Float>(t15822: F, t3160: F, t1065: F, t2852: F, t3173: F, t4879: F, t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F) -> (F, F, F, F, F, F, F) {
    let t15932 = t15822 * t3160;
    let t15935 = t1065 * t2852;
    let t15942 = F::cast_from(0.28582678745379824648e-3_f64) * t4879 * t3173;
    let t15957 = t4866 * t73;
    let t15984 = t11710 * t4782;
    let t15986 = F::cast_from(0.19055119163586549765e-3_f64) * t3091 * t15984;
    let t15987 = t140 * t1014;
    (t15932, t15935, t15942, t15957, t15984, t15986, t15987)
}

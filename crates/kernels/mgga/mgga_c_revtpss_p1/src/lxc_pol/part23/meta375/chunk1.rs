//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1708/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1708<F: Float>(t15993: F, t4574: F, t1011: F, t1012: F, t11821: F, t11922: F, t4906: F, t3115: F, t11670: F, t4890: F, t3317: F, t3299: F) -> (F, F, F, F, F, F, F) {
    let t15994 = t15993 * t4574;
    let t15996 = t1011 * t15994 / F::new(324.0);
    let t16012 = t1012 * t11821;
    let t16035 = t11922 * t4906;
    let t16037 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t16035;
    let t16048 = t11670 * t4890;
    let t16049 = t3317 * t16048;
    let t16052 = t3299 * t16048;
    (t15996, t16012, t16035, t16037, t16048, t16049, t16052)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1462/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462<F: Float>(t1065: F, t372: F, t6299: F, t3115: F, t42793: F, t6272: F, t19675: F, t1025: F, t371: F, t6276: F, t676: F, t15749: F, t4858: F) -> (F, F, F, F, F) {
    let t66777 = t372 * t1065 * t6299;
    let t67015 = t3115 * t42793 * t6272;
    let t67052 = t372 * t19675;
    let t67186 = t1025 * t371 * t676 * t6276;
    let t67195 = t4858 * t15749;
    (t66777, t67015, t67052, t67186, t67195)
}

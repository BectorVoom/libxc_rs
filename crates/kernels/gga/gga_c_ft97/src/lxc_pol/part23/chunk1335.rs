//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1335/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1335<F: Float>(t15128: F, t28847: F, t1882: F, t31816: F, t31706: F, t112952: F, t113932: F, t114142: F, t114162: F, t114271: F, t114907: F, t125586: F, t15460: F, t18997: F, t19002: F, t19006: F, t1901: F, t19324: F, t19593: F, t19617: F, t19622: F, t19626: F, t19630: F, t25271: F, t296: F, t446: F, t53797: F, t54032: F, t99672: F) -> (F, F) {
    let t126646 = t15128 * t28847;
    let t126653 = t1882 * t31816;
    let t126655 = t1882 * t31706;
    let t126685 = 4.0 / 3.0 * t446 * t296 * t126646 + 4.0 / 3.0 * t446 * t296 * t125586 + 2.0 / 9.0 * t126653 + t126655 / 9.0 + 4.0 / 9.0 * t53797 * t99672 * t19593 + 4.0 / 9.0 * t53797 * t114271 * t19617 + 4.0 / 9.0 * t53797 * t112952 * t19622 - 4.0 / 27.0 * t54032 * t112952 * t19626 + 4.0 / 3.0 * t53797 * t114907 * t18997 + 4.0 / 9.0 * t53797 * t99672 * t19630 + 8.0 / 9.0 * t53797 * t113932 * t19002 - 8.0 / 27.0 * t54032 * t113932 * t19006 - 4.0 / 3.0 * t1901 * t15460 * t25271 * t19324 - t114142 - t114162;
    (t126646, t126685)
}

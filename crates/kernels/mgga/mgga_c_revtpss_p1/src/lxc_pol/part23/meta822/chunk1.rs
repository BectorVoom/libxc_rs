//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2674/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674<F: Float>(t11922: F, t15906: F, t19753: F, t20090: F, t3115: F, t19649: F, t372: F, t11774: F, t20039: F, t53405: F, t19837: F, t19744: F) -> (F, F, F, F, F, F) {
    let t66288 = t15906 * t11922 * t19753;
    let t66304 = t3115 * t11922 * t20090;
    let t66306 = t372 * t19649;
    let t66328 = t11774 * t53405 * t20039;
    let t66332 = t3115 * t11922 * t19837;
    let t66355 = t3115 * t11922 * t19744;
    (t66288, t66304, t66306, t66328, t66332, t66355)
}

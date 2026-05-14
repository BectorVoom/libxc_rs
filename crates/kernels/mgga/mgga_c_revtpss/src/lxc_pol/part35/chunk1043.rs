//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1043/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1043<F: Float>(t110502: F, t25375: F, t28314: F, t99463: F, t27213: F, t28360: F, t28368: F, t99404: F, t98849: F, t30405: F, t689: F, t25431: F, t25411: F, t110275: F, t93281: F, t6049: F, t7384: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110503 = t25375 * t110502;
    let t110505 = t99463 * t28314;
    let t110517 = t27213 * t28360;
    let t110525 = t99404 * t28368;
    let t110527 = t98849 * t28368;
    let t110541 = t30405 * t689;
    let t110542 = t25431 * t110541;
    let t110544 = t25411 * t110541;
    let t110572 = t93281 * t110275;
    let t110584 = t689 * t7384 * t6049;
    (t110503, t110505, t110517, t110525, t110527, t110542, t110544, t110572, t110584)
}

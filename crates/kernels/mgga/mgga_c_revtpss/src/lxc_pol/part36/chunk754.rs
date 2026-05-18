//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 754/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk754<F: Float>(t2155: F, t3801: F, t38: F, t4173: F, t1497: F, t84: F, t77: F, t1470: F, t603: F, t1493: F, t76: F, t1937: F, t4248: F) -> (F, F, F, F, F, F) {
    let t7673 = t2155 * t3801;
    let t7702 = t4173 * t38;
    let t7705 = t84 * t1497;
    let t7706 = t77 * t7705;
    let t7709 = t603 * t1470;
    let t7719 = t76 * t1493;
    let t7731 = F::new(2.0) * t4248 * t1937;
    (t7673, t7702, t7706, t7709, t7719, t7731)
}

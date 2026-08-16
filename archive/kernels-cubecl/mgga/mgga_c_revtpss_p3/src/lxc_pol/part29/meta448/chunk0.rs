//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1677/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1677<F: Float>(t14365: F, t25759: F, t1113: F, t775: F, t2430: F, t33: F, t2408: F, t890: F, t2832: F, t4135: F, t4147: F, t112: F, t239: F) -> (F, F, F, F, F, F, F, F) {
    let t25760 = t25759 * t14365;
    let t25763 = t1113 * t775;
    let t25767 = t33 * t2430;
    let t25778 = t33 * t2408;
    let t25781 = t1113 * t890;
    let t25784 = t33 * t2832;
    let t25802 = t4147 * t4135;
    let t25821 = t239 * t112;
    (t25760, t25763, t25767, t25778, t25781, t25784, t25802, t25821)
}

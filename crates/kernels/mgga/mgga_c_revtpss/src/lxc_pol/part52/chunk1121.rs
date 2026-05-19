//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1121/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1121<F: Float>(t119836: F, t119893: F, t1032: F, t2735: F, t119867: F, t233: F, t240: F, t31838: F, t31840: F, t845: F, t31834: F, t846: F) -> (F, F, F, F, F) {
    let t119894 = t119836 * t119893;
    let t119900 = t2735 * t1032;
    let t119903 = t119900 * t233 * t240 * t119867;
    let t119912 = t31838 * t845 * t31840;
    let t119913 = F::cast_from(0.34708173928447610098e-2_f64) * t119912;
    let t119914 = t31834 * t846;
    (t119894, t119900, t119903, t119913, t119914)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1289/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1289<F: Float>(t3938: F, t9818: F, t9819: F, t9816: F, t4003: F, t4056: F, t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F) -> (F, F, F, F, F, F) {
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9840 = t4003 * t4056;
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = F::cast_from(24.0_f64) * t9342 * t521;
    (t9821, t9822, t9840, t9845, t9847, t9854)
}

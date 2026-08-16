//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 881/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk881<F: Float>(t225: F, t3727: F, t494: F, t1269: F, t460: F, t1275: F, t493: F) -> (F, F, F) {
    let t3729 = t3727 * t225 * t494;
    let t3732 = t460 * t1269;
    let t3736 = F::cast_from(1.0_f64) / t1275 / t493;
    (t3729, t3732, t3736)
}

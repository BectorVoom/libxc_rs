//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 469/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk469<F: Float>(t225: F, t2769: F, t2435: F, t871: F, t785: F, t870: F, t2439: F, t123: F, t212: F, t676: F) -> (F, F, F, F, F, F) {
    let t2770 = t225 * t2769;
    let t2776 = F::new(0.73171657588172351096e-2) * t2435 * t871;
    let t2777 = t785 * t225;
    let t2778 = t2777 * t870;
    let t2780 = F::new(0.65049603595885220126e-3) * t2439 * t2778;
    let t2782 = t123 * t676 * t212;
    (t2770, t2776, t2777, t2778, t2780, t2782)
}

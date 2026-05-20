//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 493/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk493<F: Float>(t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F, t1269: F, t460: F, t1275: F, t493: F, t225: F, t1204: F) -> (F, F, F, F, F, F) {
    let t3717 = t1209 * t1284;
    let t3718 = t3717 * t3624;
    let t3719 = t66 * t482;
    let t3720 = t828 * t3719;
    let t3732 = t460 * t1269;
    let t3736 = F::new(1.0) / t1275 / t493;
    let t3737 = t225 * t3736;
    let t3746 = t1204 * t1284;
    (t3718, t3719, t3720, t3732, t3737, t3746)
}

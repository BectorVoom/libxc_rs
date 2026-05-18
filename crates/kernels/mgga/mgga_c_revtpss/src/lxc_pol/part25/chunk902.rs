//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 902/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk902<F: Float>(t10671: F, t813: F, t2394: F, t2476: F, t236: F, t807: F, t2689: F, t2694: F, t2430: F, t854: F, t243: F, t247: F, t9949: F) -> (F, F, F, F, F, F, F) {
    let t10673 = F::new(0.12846167376791569079e-2) * t813 * t10671;
    let t10674 = t2476 * t2394;
    let t10675 = t236 * t10674;
    let t10676 = t807 * t10675;
    let t10678 = t2689 * t2694;
    let t10680 = t854 * t2430;
    let t10681 = t236 * t10680;
    let t10682 = t807 * t10681;
    let t10685 = t9949 * t243 * t247;
    (t10673, t10674, t10676, t10678, t10680, t10682, t10685)
}

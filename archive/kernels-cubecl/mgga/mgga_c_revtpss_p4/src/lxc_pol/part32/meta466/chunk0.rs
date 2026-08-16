//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1692/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1692<F: Float>(t26249: F, t3908: F, t7507: F, t786: F, t1364: F, t2097: F, t25937: F, t7282: F, t10073: F, t1426: F, t2098: F) -> (F, F, F, F, F, F, F, F) {
    let t26251 = F::cast_from(0.11565819519348392139e-2_f64) * t26249 * t3908;
    let t26252 = t786 * t7507;
    let t26253 = t26252 * t1364;
    let t26260 = t25937 * t2097;
    let t26261 = t7282 * t26260;
    let t26263 = F::cast_from(0.24093411633903331839e-3_f64) * t10073 * t26261;
    let t26264 = t2098 * t1426;
    let t26265 = t786 * t26264;
    (t26251, t26252, t26253, t26260, t26261, t26263, t26264, t26265)
}

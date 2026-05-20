//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 642/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk642<F: Float>(t225: F, t3727: F, t494: F, t1269: F, t460: F, t1275: F, t493: F, t1294: F, t1204: F, t1284: F, t1280: F, t3568: F) -> (F, F, F, F, F, F, F) {
    let t3729 = t3727 * t225 * t494;
    let t3732 = t460 * t1269;
    let t3736 = F::new(1.0) / t1275 / t493;
    let t3737 = t225 * t3736;
    let t3738 = t1294 * t1294;
    let t3739 = t3737 * t3738;
    let t3746 = t1204 * t1284;
    let t3751 = t1280 * t3568;
    (t3729, t3732, t3737, t3738, t3739, t3746, t3751)
}

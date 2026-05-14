//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 643/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk643<F: Float>(t1222: F, t1261: F, t1782: F, t1808: F, t3657: F, t3684: F, t3718: F, t464: F, t5358: F, t5363: F, t5366: F, t5373: F, t5379: F, t5381: F, t5391: F, t6653: F, t6659: F, t6663: F, t6667: F, t6673: F, t6679: F, t6683: F, t6690: F) -> (F,) {
    let t6694 = t1222 * t6653 / 216.0 + t5373 * t1782 / 54.0 - t1222 * t6659 / 288.0 - t1222 * t6663 / 144.0 - t5358 / 432.0 + 11.0 / 108.0 * t6667 * t464 - t3657 - 0.28582678745379824648e-3 * t5363 - t5366 / 54.0 + 0.23818898954483187207e-3 * t1261 * t6673 + 0.15244095330869239812e-2 * t5391 * t1808 - 0.14291339372689912324e-3 * t1261 * t6679 - 0.28582678745379824648e-3 * t1261 * t6683 - 0.28582678745379824648e-3 * t5381 * t1808 - t3684 - 0.42874018118069736972e-3 * t3718 * t6690 - 0.19055119163586549765e-3 * t5379;
    (t6694,)
}

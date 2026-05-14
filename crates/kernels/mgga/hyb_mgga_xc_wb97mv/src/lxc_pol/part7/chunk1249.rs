//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1249/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1249<F: Float>(t10762: F, t10776: F, t10785: F, t10806: F, t10820: F, t180: F, t2115: F, t2122: F, t2123: F, t2127: F, t2144: F, t22191: F, t22271: F, t26016: F, t30518: F, t30611: F, t3248: F, t4026: F, t4031: F, t4048: F, t6620: F, t6628: F, t739: F, t8774: F, t8777: F, t8802: F, t8803: F) -> (F,) {
    let t30664 = -t8774 * t10785 / 2.0 - t3248 * t30611 - t8777 * t10785 / 4.0 + 4.0 * t2127 * t10762 * t739 + 2.0 * t2127 * t4026 * t2115 + 7.0 / 2.0 * t4048 * t6620 + 15.0 / 4.0 * t10820 * t8803 - t8802 * t26016 - t10776 * t6620 / 4.0 - t22191 * t4031 * t8803 / 8.0 - 6.0 * t6628 * t4031 * t2115 - 24.0 * t10806 * t8803 + 24.0 * t22271 * t4031 * t2123 + 7.0 / 2.0 * t2144 * t10785 - 6.0 * t6628 * t4026 * t2123 - t2122 * t30518 * t180;
    (t30664,)
}

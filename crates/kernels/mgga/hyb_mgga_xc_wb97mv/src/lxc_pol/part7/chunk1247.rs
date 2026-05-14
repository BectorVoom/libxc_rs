//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1247/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1247<F: Float>(t10762: F, t2143: F, t4026: F, t6659: F, t180: F, t10776: F, t10785: F, t10806: F, t10809: F, t10815: F, t10820: F, t10826: F, t2135: F, t22200: F, t22208: F, t26016: F, t3262: F, t3263: F, t3269: F, t3275: F, t4031: F, t6620: F, t6648: F, t8803: F, t8830: F) -> (F, F) {
    let t30593 = t2143 * t10762;
    let t30598 = t6659 * t4026;
    let t30611 = t180 * t10762;
    let t30614 = 85.0 / 4.0 * t10776 * t8803 - 4.0 * t3262 * t26016 - 5.0 / 2.0 * t10806 * t6620 - 19.0 / 8.0 * t22200 * t4031 * t8803 - 4.0 * t2135 * t10762 * t3263 - 2.0 * t10809 * t6620 - 5.0 / 2.0 * t6648 * t4026 * t8803 + t30593 * t3263 / 2.0 + t10815 * t6620 / 4.0 + t30598 * t8803 / 8.0 + t3269 * t26016 / 2.0 + t10820 * t6620 / 8.0 + t22208 * t4031 * t8803 / 16.0 - 2.0 * t10826 * t26016 - t8830 * t10785 - 2.0 * t3275 * t30611;
    (t30611, t30614)
}

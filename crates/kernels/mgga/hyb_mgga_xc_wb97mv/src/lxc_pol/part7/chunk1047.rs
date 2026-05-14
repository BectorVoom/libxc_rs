//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1047/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1047<F: Float>(t4031: F, t6624: F, t180: F, t4026: F, t6648: F, t2135: F, t2143: F, t6659: F, t1264: F, t181: F, t178: F, t10762: F, t173: F, t3244: F, t3262: F, t3263: F, t3269: F, t3272: F, t3275: F, t4032: F, t746: F, t750: F, t8780: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10776 = t6624 * t4031;
    let t10785 = t180 * t4026;
    let t10806 = t6648 * t4031;
    let t10809 = t2135 * t4026;
    let t10815 = t2143 * t4026;
    let t10820 = t6659 * t4031;
    let t10823 = t1264 * t181;
    let t10826 = t178 * t1264;
    let t10836 = 15.0 / 2.0 * t4032 * t3263 - 4.0 * t3262 * t8780 - 5.0 / 2.0 * t10806 * t3263 - 2.0 * t10809 * t3263 + t746 * t10762 * t180 / 2.0 + t10815 * t3263 / 4.0 + t3269 * t8780 / 2.0 + t10820 * t3263 / 8.0 - 8.0 * t10823 * t3244 - 2.0 * t10826 * t8780 - 4.0 * t3272 * t4026 - t3275 * t10785 - 4.0 * t750 * t10762 - t173 * t10762 * t180;
    (t10776, t10785, t10806, t10809, t10815, t10820, t10823, t10826, t10836)
}

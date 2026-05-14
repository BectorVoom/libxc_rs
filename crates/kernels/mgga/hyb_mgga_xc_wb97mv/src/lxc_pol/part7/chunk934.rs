//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 934/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk934<F: Float>(t2115: F, t2122: F, t2123: F, t6624: F, t180: F, t3244: F, t1264: F, t2135: F, t6648: F, t2143: F, t6659: F, t181: F, t178: F, t173: F, t3249: F, t3262: F, t3263: F, t3269: F, t3272: F, t3275: F, t6620: F, t746: F, t750: F, t8761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8774 = t2122 * t2115;
    let t8777 = t6624 * t2123;
    let t8780 = t180 * t3244;
    let t8802 = t2122 * t1264;
    let t8803 = t180 * t2123;
    let t8806 = t2135 * t3244;
    let t8809 = t6648 * t1264;
    let t8817 = t2143 * t3244;
    let t8822 = t6659 * t1264;
    let t8825 = t2115 * t181;
    let t8830 = t178 * t2115;
    let t8838 = 15.0 / 2.0 * t8802 * t8803 - 4.0 * t8806 * t3263 - 5.0 / 2.0 * t8809 * t8803 - 2.0 * t3262 * t6620 + t746 * t8761 * t180 / 2.0 + t8817 * t3263 / 2.0 + t3269 * t6620 / 4.0 + t8822 * t8803 / 8.0 - 4.0 * t8825 * t1264 - 8.0 * t3272 * t3244 - t8830 * t3249 - 2.0 * t3275 * t8780 - 4.0 * t750 * t8761 - t173 * t8761 * t180;
    (t8774, t8777, t8780, t8802, t8803, t8809, t8822, t8825, t8830, t8838)
}

//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 906/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk906<F: Float>(t1173: F, t6147: F, t1861: F, t8228: F, t6155: F, t3025: F, t1859: F, t3: F, t544: F, t1877: F, t3026: F, t3031: F, t1867: F, t3032: F, t39: F, t573: F) -> (F, F, F, F, F, F, F, F) {
    let t8229 = t6147 * t1173;
    let t8231 = t8228 * t8229 * t1861;
    let t8234 = t6155 * t1173;
    let t8236 = t3025 * t8234 * t1861;
    let t8239 = t1859 * t3;
    let t8241 = t3025 * t8239 * t544;
    let t8245 = t3025 * t3026 * t1877;
    let t8249 = t3031 * t3026 * t1861;
    let t8252 = t1867 * t3;
    let t8254 = t3031 * t8252 * t544;
    let t8258 = t3031 * t3032 * t1877;
    let t8261 = t573 * t39;
    (t8231, t8236, t8241, t8245, t8249, t8254, t8258, t8261)
}

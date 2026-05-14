//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1299/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1299<F: Float>(t132: F, t1874: F, t29908: F, t10621: F, t11269: F, t11274: F, t2013: F, t2039: F, t23152: F, t2456: F, t27052: F, t2708: F, t3480: F, t3979: F, t3988: F, t453: F, t6175: F, t674: F, t7198: F, t9276: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t31787 = t29908 * t1874;
    let t31808 = piecewise3(t133, 0.0, 280.0 / 81.0 * t23152 * t3979 * t2039 + 224.0 / 27.0 * t9276 * t31787 - 28.0 / 27.0 * t11269 * t2013 + 32.0 / 9.0 * t2456 * t453 * t2708 - 16.0 / 9.0 * t3480 * t1874 + 16.0 / 3.0 * t3480 * t6175 - 28.0 / 27.0 * t7198 * t3988 * t2039 + 8.0 / 9.0 * t2456 * t10621 * t674 + 4.0 / 9.0 * t11274 * t2013 + t27052);
    (t31787, t31808)
}

//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1166/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1166<F: Float>(t21425: F, t35: F, t8262: F, t33: F, t1859: F, t8261: F, t1173: F, t544: F, t1856: F, t39: F, t6155: F, t6144: F, t6147: F, t81: F, t8344: F, t1205: F, t6261: F) -> (F, F, F, F, F, F, F, F) {
    let t24954 = t35 * t21425 * t8262;
    let t24958 = t33 * param_hyb_omega_0;
    let t24962 = t8261 * t1859;
    let t24963 = t1173 * t544;
    let t24969 = t1856 * t39 * t6155;
    let t24974 = t6144 * t39 * t6147;
    let t25201 = t81 * t8344;
    let t25207 = t6261 * t1205;
    (t24954, t24958, t24962, t24963, t24969, t24974, t25201, t25207)
}

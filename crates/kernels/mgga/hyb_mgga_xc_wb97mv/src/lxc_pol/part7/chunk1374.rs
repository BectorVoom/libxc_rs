//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1374/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1374<F: Float>(t10201: F, t11241: F, t11242: F, t11577: F, t12105: F, t33958: F, t4: F, t9247: F, t9249: F, t9612: F, t9708: F, t9709: F, t9711: F, t9712: F, t9713: F, t29023: F) -> (F,) {
    let t33964 = t4 * t33958 + 2.0 * t10201 + 2.0 * t11241 + 2.0 * t11242 + 2.0 * t11577 + 2.0 * t12105 + 4.0 * t9247 + 2.0 * t9249 + 2.0 * t9612 + 2.0 * t9708 + 4.0 * t9709 + 2.0 * t9711 + 2.0 * t9712 + 4.0 * t9713;
    let tv4rho42 = t29023 + t33964;
    (tv4rho42,)
}

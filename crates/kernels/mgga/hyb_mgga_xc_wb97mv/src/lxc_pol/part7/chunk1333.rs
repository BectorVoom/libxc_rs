//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1333/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1333<F: Float>(t2775: F, t4509: F, t458: F, t23867: F, t23870: F, t23874: F, t23878: F, t23882: F, t23885: F, t23888: F, t24019: F, t24022: F, t24024: F, t24026: F, t24028: F, t24036: F, t24075: F, t24077: F, t24080: F, t27854: F) -> (F,) {
    let t32561 = t458 * t4509 * t2775;
    let t32562 = 12.0 * t24019 - t23867 + 2.0 * t24022 + 120.0 * t24024 + 20.0 * t24026 - t23870 + t23874 + t23878 + t23882 - t23885 - 8.0 * t24028 - 8.0 * t24036 - 24.0 * t24075 + 0.2077903092681775651e3 * t27854 + 32.0 * t24077 + t24080 + t23888 + t32561;
    (t32562,)
}

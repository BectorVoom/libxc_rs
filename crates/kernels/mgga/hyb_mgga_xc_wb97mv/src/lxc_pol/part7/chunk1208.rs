//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1208/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1208<F: Float>(t1839: F, t3856: F, t1175: F, t8206: F, t10280: F, t1993: F, t6461: F, t10246: F, t2007: F, t554: F, t21885: F, t3865: F, t3873: F, t6432: F, t3: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t29097 = t3856 * t1839;
    let t29099 = t1175 * t8206;
    let t29105 = t1993 * t6461 * t10280;
    let t29109 = t554 * t2007 * t10246;
    let t29112 = t1993 * t21885 * t3865;
    let t29119 = t554 * t6432 * t3873;
    let t29125 = t544 * t3;
    (t29097, t29099, t29105, t29109, t29112, t29119, t29125)
}

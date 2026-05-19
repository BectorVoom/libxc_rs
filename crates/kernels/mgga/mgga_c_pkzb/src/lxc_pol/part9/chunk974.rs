//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 974/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk974<F: Float>(t2099: F, t2918: F, t757: F, t2946: F, t300: F, t2107: F, t1123: F, t779: F, t2029: F, t759: F, t2106: F, t178: F, t5711: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7692 = t2099 * t2918;
    let t7694 = F::cast_from(0.28582678745379824648e-3_f64) * t757 * t7692;
    let t7695 = t300 * t2946;
    let t7696 = t7695 * t2107;
    let t7699 = t779 * t1123;
    let t7700 = t300 * t7699;
    let t7701 = t2029 * t759;
    let t7702 = t7701 * t2106;
    let t7703 = t7700 * t7702;
    let t7706 = t5711 * t178;
    (t7692, t7694, t7695, t7696, t7699, t7700, t7701, t7702, t7703, t7706)
}

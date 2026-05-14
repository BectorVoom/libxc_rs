//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 314/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk314<F: Float>(t322: F, t343: F, t352: F, t838: F, t839: F, t841: F, t843: F, t845: F, t847: F, t849: F, t855: F, t856: F, t758: F, t761: F) -> (F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t860 = piecewise5(t323, t838, t331, -0.64e0 * t839 - 0.8704e0 * t841 - 0.4607056813647e1 * t843 + 0.122462410087e2 * t845 - 0.957855118103e1 * t847 + 0.3101306810232e1 * t849 - 0.362942158544e0 * t343 * t839, -0.105e1 * t855 * t856 * t352);
    let t862 = t758 * t761;
    (t860, t862)
}

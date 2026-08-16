//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 273/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk273<F: Float>(t322: F, t828: F, t343: F, t352: F, t838: F, t839: F, t841: F, t843: F, t845: F, t847: F, t849: F, t855: F, t758: F, t761: F) -> (F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t856 = piecewise3::<F>(t332, t828, F::cast_from(0.0_f64));
    let t860 = piecewise5::<F>(t323, t838, t331, -F::cast_from(0.64e0_f64) * t839 - F::cast_from(0.8704e0_f64) * t841 - F::cast_from(0.4607056813647e1_f64) * t843 + F::cast_from(0.122462410087e2_f64) * t845 - F::cast_from(0.957855118103e1_f64) * t847 + F::cast_from(0.3101306810232e1_f64) * t849 - F::cast_from(0.362942158544e0_f64) * t343 * t839, -F::cast_from(0.105e1_f64) * t855 * t856 * t352);
    let t862 = t758 * t761;
    (t856, t860, t862)
}

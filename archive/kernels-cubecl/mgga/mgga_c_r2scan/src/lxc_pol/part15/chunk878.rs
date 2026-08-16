//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 878/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk878<F: Float>(t2122: F, t2139: F, t2196: F, t2557: F, t2582: F, t5108: F, t6260: F, t6266: F, t6268: F, t7934: F, t7939: F, t7941: F, t7946: F, t7951: F, t7953: F, t7956: F, t7961: F, t7964: F, t7968: F, t7970: F, t7974: F, t7979: F) -> F {
    let t7982 = -F::cast_from(0.27439371595564631661e-1_f64) * t2557 * t7934 + t7939 - F::cast_from(0.43341108700271342816e-1_f64) * t2582 * t7941 + F::cast_from(0.13002332610081402845e0_f64) * t2139 * t7946 + t7951 - t6260 + F::cast_from(0.10401866088065122276e1_f64) * t2196 * t7953 + F::cast_from(0.2600466522016280569e0_f64) * t2139 * t7956 + t6266 - F::cast_from(0.1358426014257923078e0_f64) * t6268 - F::cast_from(0.34930954652346593434e-1_f64) * t7961 - F::cast_from(0.2600466522016280569e0_f64) * t5108 * t7964 + t7968 + F::cast_from(0.54878743191129263322e-1_f64) * t2122 * t7970 - F::cast_from(0.27439371595564631661e-1_f64) * t2557 * t7974 + F::cast_from(0.2600466522016280569e0_f64) * t2139 * t7979;
    t7982
}

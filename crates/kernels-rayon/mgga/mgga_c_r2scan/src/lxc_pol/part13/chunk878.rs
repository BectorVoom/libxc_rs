//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 878/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk878(t2122: f64, t2139: f64, t2196: f64, t2557: f64, t2582: f64, t5108: f64, t6260: f64, t6266: f64, t6268: f64, t7934: f64, t7939: f64, t7941: f64, t7946: f64, t7951: f64, t7953: f64, t7956: f64, t7961: f64, t7964: f64, t7968: f64, t7970: f64, t7974: f64, t7979: f64) -> f64 {
    let t7982 = -0.27439371595564631661e-1_f64 * t2557 * t7934 + t7939 - 0.43341108700271342816e-1_f64 * t2582 * t7941 + 0.13002332610081402845e0_f64 * t2139 * t7946 + t7951 - t6260 + 0.10401866088065122276e1_f64 * t2196 * t7953 + 0.2600466522016280569e0_f64 * t2139 * t7956 + t6266 - 0.1358426014257923078e0_f64 * t6268 - 0.34930954652346593434e-1_f64 * t7961 - 0.2600466522016280569e0_f64 * t5108 * t7964 + t7968 + 0.54878743191129263322e-1_f64 * t2122 * t7970 - 0.27439371595564631661e-1_f64 * t2557 * t7974 + 0.2600466522016280569e0_f64 * t2139 * t7979;
    t7982
}

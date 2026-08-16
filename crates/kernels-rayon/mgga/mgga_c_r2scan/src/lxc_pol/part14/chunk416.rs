//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 416/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk416(t732: f64, t745: f64, t1419: f64, t230: f64, t1422: f64, t1376: f64, t1650: f64, t1651: f64, t1655: f64, t1662: f64, t1667: f64, t1671: f64, t1674: f64, t1688: f64, t1695: f64, t1696: f64, t216: f64, t236: f64, t596: f64, t598: f64) -> (f64, f64, f64) {
    let t1699 = t732 * t745;
    let t1702 = 12.0_f64 * t1419 * t230;
    let t1704 = 32.0_f64 * t1422 * t230;
    let t1705 = t1650 - 0.675260332e-1_f64 * t1651 * t598 - 0.1350520664e0_f64 * t596 * t1655 - t1662 + t1667 - t1671 + 0.16936279733333333333e-2_f64 * t1674 + t1688 - 0.21973736767207854065e-2_f64 * t1376 * t216 - t1695 + 0.5848223622634646207e0_f64 * t1696 * t236 + 0.11696447245269292414e1_f64 * t1699 - t1702 + t1704;
    (t1699, t1702, t1705)
}

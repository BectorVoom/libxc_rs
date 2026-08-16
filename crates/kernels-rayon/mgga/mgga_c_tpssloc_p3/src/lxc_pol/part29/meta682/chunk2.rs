//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2306/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306(t15590: f64, t7338: f64, t27614: f64, t3572: f64, t27617: f64, t3523: f64, t1218: f64, t15531: f64, t15535: f64, t15622: f64, t15627: f64, t15631: f64, t15637: f64, t24729: f64, t24733: f64, t4984: f64, t86120: f64, t86146: f64, t86164: f64, t86167: f64, t86171: f64) -> f64 {
    let t95238 = t15590 * t7338;
    let t95242 = t27614 * t3572 / 1152.0_f64;
    let t95244 = t27617 * t3523 / 1728.0_f64;
    let t95260 = t95238 * t1218 / 768.0_f64 + t95242 - t95244 + t24729 * t15622 / 768.0_f64 + t86146 * t15627 / 256.0_f64 - t86164 * t15631 / 256.0_f64 - t86167 * t4984 / 768.0_f64 - t24733 * t15637 / 768.0_f64 - t24733 * t15531 / 1536.0_f64 + t86171 * t15535 / 1536.0_f64 + 5.0_f64 / 10368.0_f64 * t86120;
    t95260
}

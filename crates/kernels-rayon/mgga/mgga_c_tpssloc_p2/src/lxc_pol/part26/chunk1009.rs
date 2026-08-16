//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1009/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1009(t11853: f64, t1214: f64, t248: f64, t11616: f64, t68: f64, t484: f64, t10913: f64, t4972: f64, t4582: f64, t1174: f64, t11821: f64, t11825: f64, t11834: f64, t11836: f64, t11839: f64, t11842: f64, t11845: f64, t11850: f64, t1213: f64, t1227: f64, t1232: f64, t3490: f64, t3527: f64, t3531: f64, t3587: f64, t488: f64) -> (f64, f64, f64, f64) {
    let t11855 = t248 * t1214 * t11853;
    let t11858 = t11616 * t68;
    let t11859 = t11858 * t484;
    let t11862 = t4972 * t10913;
    let t11863 = t4582 * t11862;
    let t11866 = -t11821 / 4608.0_f64 + 5.0_f64 / 4608.0_f64 * t3490 * t3587 - t11825 * t1232 / 1536.0_f64 - t3490 * t3527 / 1536.0_f64 - t3490 * t3531 / 768.0_f64 + t11834 + t11836 / 432.0_f64 - t11839 / 288.0_f64 - t11842 / 144.0_f64 - t1174 * t11845 / 288.0_f64 - t1174 * t11850 / 48.0_f64 + t1213 * t11855 / 3072.0_f64 + t11859 * t488 / 3072.0_f64 - t1227 * t11863 / 768.0_f64;
    (t11855, t11858, t11863, t11866)
}

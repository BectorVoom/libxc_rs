//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2167/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2167(t15892: f64, t2535: f64, t2528: f64, t40225: f64, t15921: f64, t588: f64, t15971: f64, t12364: f64, t5234: f64, t1354: f64, t12365: f64, t5289: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54469 = t15892 * t2535;
    let t54470 = 0.17544670867903938621e1_f64 * t54469;
    let t54471 = t15892 * t2528;
    let t54472 = 0.51947577317044391276e2_f64 * t54471;
    let t54473 = 36.0_f64 * t40225;
    let t54475 = 24.0_f64 * t588 * t15921;
    let t54477 = t588 * t15971;
    let t54478 = 12.0_f64 * t54477;
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54534 = 119.0_f64 / 4608.0_f64 * t54533;
    let t54555 = t12365 * t5289;
    (t54470, t54472, t54473, t54475, t54478, t54532, t54534, t54555)
}

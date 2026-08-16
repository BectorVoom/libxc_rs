//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 792/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk792(t2321: f64, t955: f64, t6897: f64, t986: f64, t1048: f64, t2330: f64, t1543: f64, t2854: f64, t2858: f64, t4987: f64, t4938: f64, t889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6966 = t2321 * t955;
    let t6967 = t986 * t6897;
    let t6969 = t1048 * t6967 * t2330;
    let t6970 = 2.0_f64 * t6969;
    let t6972 = t2858 * t2854 * t1543;
    let t6973 = 6.0_f64 * t6972;
    let t6975 = 0.34631718211362927518e2_f64 * t4987;
    let t6976 = t4938 * t889;
    (t6966, t6967, t6970, t6973, t6975, t6976)
}

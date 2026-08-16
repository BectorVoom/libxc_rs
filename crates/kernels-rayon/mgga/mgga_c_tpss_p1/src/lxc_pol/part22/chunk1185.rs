//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1185/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1185(t10514: f64, t17930: f64, t580: f64, t750: f64, t2133: f64, t30: f64, t159: f64, t2138: f64, t1695: f64, t212: f64, t223: f64, t5543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17931 = t17930 * t10514;
    let t17934 = t580 * t750;
    let t17938 = t30 * t2133;
    let t17942 = t2138 * t159;
    let t17944 = t17942 * t212 * t1695;
    let t17946 = t5543 * t223;
    (t17931, t17934, t17938, t17942, t17944, t17946)
}

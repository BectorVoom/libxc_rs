//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 603/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk603(t1064: f64, t6330: f64, t1079: f64, t1056: f64, t6334: f64, t6326: f64, t945: f64, t104: f64, t111: f64, t120: f64, t3061: f64, t3150: f64, t3153: f64, t3159: f64, t4889: f64, t6272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6452 = t1064 * t6330;
    let t6455 = t1079 * t6330;
    let t6458 = t1056 * t6330;
    let t6461 = t1064 * t6334;
    let t6464 = t945 * t6326;
    let t6467 = t1079 * t6334;
    let t6470 = t1056 * t6326;
    let t6477 = 0.9368e-2_f64 * t4889 - 0.1585e-2_f64 * t111 * t6452 - 0.10082625e-4_f64 * t120 * t6455 + 0.7026e-2_f64 * t104 * t6458 + 0.7925e-3_f64 * t111 * t6461 - 0.52833333333333333333e-3_f64 * t111 * t6464 + 0.50413125e-5_f64 * t120 * t6467 - 0.672175e-5_f64 * t120 * t6470 - 0.23911438650126355246e-1_f64 * t3061 * t6272 + 0.15538616723388920628e-3_f64 * t3150 * t6272 - t3153 + t3159;
    (t6452, t6455, t6458, t6461, t6464, t6467, t6470, t6477)
}

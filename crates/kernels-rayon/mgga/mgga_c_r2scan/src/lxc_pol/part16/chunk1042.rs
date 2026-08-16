//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1042/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1042(t3071: f64, t6212: f64, t3056: f64, t560: f64, t113: f64, t28335: f64, t28390: f64, t2892: f64, t146: f64, t5094: f64, t978: f64, t3053: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30428 = t6212 * t3071;
    let t30468 = t3056 * t560;
    let t30628 = t3071 * t560;
    let t30637 = t28335 * t113;
    let t30643 = t28390 * t113;
    let t30691 = t6212 * t2892;
    let t30792 = t146 * t5094 * t978;
    let t30856 = t3053 * t481;
    (t30428, t30468, t30628, t30637, t30643, t30691, t30792, t30856)
}

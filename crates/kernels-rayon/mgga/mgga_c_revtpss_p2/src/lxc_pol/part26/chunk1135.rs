//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1135/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1135(t1113: f64, t2832: f64, t3351: f64, t775: f64, t2430: f64, t11061: f64, t33: f64, t2408: f64, t10489: f64, t47672: f64, t9590: f64, t2453: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94286 = t1113 * t2832;
    let t94293 = t3351 * t775;
    let t94297 = t1113 * t2430;
    let t94312 = t33 * t11061;
    let t94316 = t1113 * t2408;
    let t94320 = t33 * t10489;
    let t94349 = t47672 * t9590;
    let t94382 = t2453 * t555;
    (t94286, t94293, t94297, t94312, t94316, t94320, t94349, t94382)
}

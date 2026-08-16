//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 653/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk653(t3302: f64, t3603: f64, t3781: f64, t487: f64, t460: f64, t1248: f64, t471: f64, t670: f64, t93: f64, t198: f64, t530: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5464 = t3302 * t3603;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5479 = t3302 * t1248;
    let t5480 = t5479 * t471;
    let t5523 = t93 * t670;
    let t5536 = t198 * t530;
    let t5541 = t198 * t532;
    (t5464, t5477, t5478, t5480, t5523, t5536, t5541)
}

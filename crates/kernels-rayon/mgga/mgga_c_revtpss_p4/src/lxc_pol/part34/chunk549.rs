//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 549/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk549(t460: f64, t5462: f64, t3302: f64, t3603: f64, t3781: f64, t487: f64, t1811: f64, t473: f64, t1450: f64, t1907: f64, t198: f64, t530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5486 = t473 * t1811;
    let t5532 = t1907 * t1450;
    let t5536 = t198 * t530;
    (t5463, t5464, t5477, t5478, t5486, t5532, t5536)
}

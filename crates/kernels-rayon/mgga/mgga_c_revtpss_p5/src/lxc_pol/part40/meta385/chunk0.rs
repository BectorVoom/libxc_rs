//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1385/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1385(t3379: f64, t5105: f64, t12327: f64, t1723: f64, t3391: f64, t12331: f64, t3390: f64, t5079: f64, t1134: f64, t3399: f64, t5071: f64, t3407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16846 = 2.0_f64 * t3379 * t5105;
    let t16851 = t12327 * t1723;
    let t16852 = t16851 * t3391;
    let t16854 = t12331 * t1723;
    let t16855 = t16854 * t3391;
    let t16857 = t3390 * t5079;
    let t16858 = t16857 * t1134;
    let t16860 = t5071 * t3399;
    let t16862 = t3407 * t5079;
    (t16846, t16852, t16855, t16858, t16860, t16862)
}

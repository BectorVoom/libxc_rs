//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 605/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk605(t3863: f64, t521: f64, t1320: f64, t1333: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3827: f64, t3828: f64, t3829: f64, t3852: f64, t3854: f64, t3856: f64, t3859: f64, t3862: f64) -> (f64, f64, f64) {
    let t3865 = 32.0_f64 * t3863 * t521;
    let t3867 = 8.0_f64 * t1320 * t1333;
    let t3868 = 6.0_f64 * t198 * t3828 * t3829 - t2522 - t2562 - t2569 + t2579 + t2587 - t3827 + t3852 + t3854 + t3856 + t3859 + t3862 - t3865 - t3867;
    (t3865, t3867, t3868)
}

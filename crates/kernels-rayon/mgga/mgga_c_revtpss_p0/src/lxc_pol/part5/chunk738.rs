//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 738/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk738(t342: f64, t4995: f64, t1043: f64, t3302: f64, t357: f64, t4893: f64, t1678: f64, t359: f64, t999: f64, t1089: f64, t380: f64, t4930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    let t4999 = t4893 * t4998;
    let t5004 = t359 * t1678;
    let t5005 = t5004 * t999;
    let t5009 = t1678 * t1043 * t1089;
    let t5012 = t380 * t4930;
    (t4996, t4998, t4999, t5004, t5005, t5009, t5012)
}

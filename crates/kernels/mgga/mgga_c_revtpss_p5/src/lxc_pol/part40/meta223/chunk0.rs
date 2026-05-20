//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 879/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk879<F: Float>(t342: F, t4995: F, t1043: F, t3302: F, t357: F, t4893: F, t1678: F, t359: F, t999: F, t1089: F, t380: F, t4930: F) -> (F, F, F, F, F, F, F) {
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

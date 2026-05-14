//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 734/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk734<F: Float>(t1043: F, t4982: F, t4893: F, t1071: F, t1089: F, t1668: F, t378: F, t4866: F, t3316: F, t342: F, t3302: F, t357: F, t1678: F, t359: F, t999: F, t380: F, t4930: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4983 = t4982 * t1043;
    let t4984 = t4893 * t4983;
    let t4988 = t1071 * t1668 * t1089;
    let t4992 = t378 * t4866 * t1089;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    let t4999 = t4893 * t4998;
    let t5004 = t359 * t1678;
    let t5005 = t5004 * t999;
    let t5009 = t1678 * t1043 * t1089;
    let t5012 = t380 * t4930;
    (t4983, t4984, t4988, t4992, t4995, t4996, t4998, t4999, t5004, t5005, t5009, t5012)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1025/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1025<F: Float>(t120066: F, t120070: F, t120073: F, t121920: F, t32474: F, t119971: F, t121834: F, t251: F, t31837: F, t2061: F, t7063: F, t25410: F, t25413: F, t120111: F, t120114: F, t120117: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t121990 = 0.14932895752263002547e-1 * t120066;
    let t121991 = 0.40155686056505553065e-3 * t120070;
    let t121992 = 0.71396809808466873356e-3 * t120073;
    let t121993 = t32474 * t121920;
    let t121998 = 0.6019057092162847523e-2 * t119971 * t251 * t31837 * t121834;
    let t122002 = t7063 * t2061;
    let t122003 = t122002 * t25410;
    let t122004 = t122003 * t25413;
    let t122008 = 0.7437465841810202164e-5 * t120111;
    let t122009 = 0.39671442800215618342e-4 * t120114;
    let t122010 = 0.47023883532522246276e-4 * t120117;
    (t121990, t121991, t121992, t121993, t121998, t122002, t122003, t122004, t122008, t122009, t122010)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 953/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk953<F: Float>(t2014: F, t28176: F, t32098: F, t28043: F, t8634: F, t28019: F, t4147: F, t2034: F, t33594: F, t7235: F, t5591: F, t32119: F, t32297: F, t5542: F, t33657: F, t32114: F, t7898: F) -> (F, F, F, F, F, F, F, F) {
    let t125415 = 3.0 * t2014 * t32098 * t28176;
    let t125417 = 4.0 * t8634 * t28043;
    let t125428 = t4147 * t28019;
    let t125431 = 2.0 * t2014 * t2034 * t125428;
    let t125432 = t7235 * t33594;
    let t125453 = t4147 * t5591;
    let t125456 = 3.0 * t2014 * t32119 * t125453;
    let t125470 = t2014 * t32297 * t5542;
    let t125472 = 3.0 * t7235 * t33657;
    let t125474 = 2.0 * t7898 * t32114;
    (t125415, t125417, t125431, t125432, t125456, t125470, t125472, t125474)
}

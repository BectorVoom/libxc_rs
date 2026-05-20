//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1088/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1088<F: Float>(t28063: F, t8634: F, t2014: F, t28176: F, t32098: F, t28043: F, t28019: F, t4147: F, t2034: F, t33594: F, t7235: F, t5591: F) -> (F, F, F, F, F, F) {
    let t125409 = F::new(4.0) * t8634 * t28063;
    let t125415 = F::new(3.0) * t2014 * t32098 * t28176;
    let t125417 = F::new(4.0) * t8634 * t28043;
    let t125428 = t4147 * t28019;
    let t125431 = F::new(2.0) * t2014 * t2034 * t125428;
    let t125432 = t7235 * t33594;
    let t125453 = t4147 * t5591;
    (t125409, t125415, t125417, t125431, t125432, t125453)
}

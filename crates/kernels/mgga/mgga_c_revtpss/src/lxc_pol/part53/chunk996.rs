//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 996/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk996<F: Float>(t1936: F, t27830: F, t651: F, t1937: F, t97622: F, t108120: F, t28030: F, t6993: F, t4147: F, t5591: F, t2014: F, t32119: F, t28043: F, t6985: F, t28182: F, t8568: F) -> (F, F, F, F, F, F, F) {
    let t125436 = t651 * t27830 * t1936;
    let t125438 = t97622 * t1937;
    let t125442 = t108120 * t1937;
    let t125444 = t28030 * t6993;
    let t125453 = t4147 * t5591;
    let t125456 = 3.0 * t2014 * t32119 * t125453;
    let t125459 = t6985 * t28043;
    let t125467 = t8568 * t28182;
    (t125436, t125438, t125442, t125444, t125456, t125459, t125467)
}

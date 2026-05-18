//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1132/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1132<F: Float>(t33591: F, t4254: F, t1936: F, t27830: F, t651: F, t1937: F, t97622: F, t108120: F, t28030: F, t6993: F, t4147: F, t5591: F) -> (F, F, F, F, F, F) {
    let t125433 = t4254 * t33591;
    let t125436 = t651 * t27830 * t1936;
    let t125438 = t97622 * t1937;
    let t125442 = t108120 * t1937;
    let t125444 = t28030 * t6993;
    let t125453 = t4147 * t5591;
    (t125433, t125436, t125438, t125442, t125444, t125453)
}

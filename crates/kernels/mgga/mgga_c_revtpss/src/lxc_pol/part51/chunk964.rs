//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 964/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk964<F: Float>(t125428: F, t2014: F, t2034: F, t33594: F, t7235: F, t33591: F, t4254: F, t1936: F, t27830: F, t651: F, t1937: F, t97622: F, t119535: F, t125350: F, t125402: F, t125405: F, t125407: F, t125409: F, t125410: F, t125415: F, t125417: F, t125420: F, t1502: F, t1519: F, t32095: F, t32162: F, t4257: F, t4297: F) -> (F,) {
    let t125431 = 2.0 * t2014 * t2034 * t125428;
    let t125432 = t7235 * t33594;
    let t125433 = t4254 * t33591;
    let t125436 = t651 * t27830 * t1936;
    let t125438 = t97622 * t1937;
    let t125440 = -2.0 * t119535 * t1519 - 2.0 * t125350 * t1519 - t1502 * t32095 - 2.0 * t32162 * t4257 - 2.0 * t32162 * t4297 + 6.0 * t125402 - t125405 - t125407 - t125409 + 6.0 * t125410 + t125415 - t125417 - 4.0 * t125420 - t125431 - t125432 - 4.0 * t125433 - 4.0 * t125436 - 4.0 * t125438;
    (t125440,)
}

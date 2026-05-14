//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1169/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1169<F: Float>(t2172: F, t4153: F, t27110: F, t571: F, t13226: F, t13250: F, t1456: F, t1458: F, t1464: F, t2168: F, t27090: F, t3: F, t4154: F, t4168: F, t575: F, t7691: F, t7700: F, t96684: F, t96690: F, t96692: F, t96694: F, t97567: F, t97576: F) -> (F,) {
    let t97580 = t4153 * t2172;
    let t97586 = t571 * t27110;
    let tv4rho3sigma2 = t3 * t575 * t97567 + t13226 * t2172 + t13250 * t2168 + 3.0 * t1456 * t27110 + t1458 * t97576 + 3.0 * t1464 * t27090 + 3.0 * t4154 * t7700 + 3.0 * t4168 * t7691 + 6.0 * t96684 + 6.0 * t96690 + 3.0 * t96692 + 3.0 * t96694 + 3.0 * t97580 + 3.0 * t97586;
    (tv4rho3sigma2,)
}

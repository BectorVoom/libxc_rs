//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2069/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2069<F: Float>(t2155: F, t44126: F, t2172: F, t4153: F, t27110: F, t571: F, t27833: F, t7316: F, t13426: F, t7003: F, t18227: F, t25861: F, t4248: F) -> (F, F, F, F, F, F, F) {
    let t97498 = t2155 * t44126;
    let t97580 = t4153 * t2172;
    let t97586 = t571 * t27110;
    let t97604 = F::new(2.0) * t27833 * t7316;
    let t97606 = F::new(4.0) * t13426 * t7003;
    let t97608 = F::new(4.0) * t18227 * t7003;
    let t97610 = F::new(4.0) * t4248 * t25861;
    (t97498, t97580, t97586, t97604, t97606, t97608, t97610)
}

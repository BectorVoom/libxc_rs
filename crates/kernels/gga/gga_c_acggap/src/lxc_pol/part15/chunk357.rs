//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 357/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk357<F: Float>(t43: F, t50: F, t560: F, t1690: F, t1694: F, t292: F, t817: F, t1699: F, t1702: F, t296: F, t829: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1717 = t560 * t560;
    let t1726 = piecewise3(t44, 0.0, -2.0 / 9.0 * t817 * t1690 + 2.0 / 3.0 * t292 * t1694);
    let t1732 = piecewise3(t51, 0.0, -2.0 / 9.0 * t829 * t1699 + 2.0 / 3.0 * t296 * t1702);
    let t1734 = t1726 / 2.0 + t1732 / 2.0;
    (t1717, t1734)
}

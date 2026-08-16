//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 833/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk833<F: Float>(t598: F, t9691: F, t1891: F, t2001: F, t1896: F, t1901: F, t1734: F, t599: F, t142: F, t2030: F, t1795: F, t604: F) -> (F, F, F, F, F, F, F, F) {
    let t9692 = t598 * t9691;
    let t9694 = t2001 * t1891;
    let t9696 = t2001 * t1896;
    let t9698 = t2001 * t1901;
    let t9700 = t599 * t1734;
    let t9701 = t142 * t9700;
    let t9702 = t2030 * t9701;
    let t9704 = t604 * t1795;
    (t9692, t9694, t9696, t9698, t9700, t9701, t9702, t9704)
}

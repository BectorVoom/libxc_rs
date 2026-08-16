//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 924/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk924<F: Float>(t1359: F, t9292: F, t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t3957: F, t3961: F, t124: F, t9628: F, t800: F) -> (F, F, F, F, F, F, F) {
    let t9691 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = F::cast_from(0.30356481678079769392e-1_f64) * t1362 * t9692;
    let t9695 = t3911 * t3920;
    let t9697 = t3957 * t3961;
    let t9699 = t124 * t9628;
    let t9700 = t800 * t9699;
    (t9691, t9692, t9694, t9695, t9697, t9699, t9700)
}

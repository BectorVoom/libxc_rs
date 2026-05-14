//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 392/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk392<F: Float>(t150: F, t1603: F, t187: F, t119: F, t545: F, t557: F, t857: F, t322: F, t556: F, t449: F) -> (F, F, F, F) {
    let t1605 = t1603 * t150 * t187;
    let t1608 = t119 * t545;
    let t1611 = t857 * t557;
    let t1613 = t556 * t322;
    let t1614 = t449 * t1613;
    (t1605, t1608, t1611, t1614)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 180/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk180<F: Float>(t150: F, t545: F, t187: F, t456: F, t525: F, t182: F, t119: F, t151: F, t451: F, t455: F) -> (F, F, F, F) {
    let t546 = t545 * t150;
    let t547 = t546 * t187;
    let t550 = t456 * t525;
    let t553 = t182 * t545;
    let t556 = t451 - t455 - F::new(0.65854491829355115987e0) * t151 * t550 + F::new(0.65854491829355115987e0) * t119 * t553;
    (t547, t550, t553, t556)
}

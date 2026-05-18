//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 851/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk851<F: Float>(t11882: F, t1159: F, t848: F, t182: F, t862: F, t1016: F, t360: F, t1083: F, t171: F) -> (F, F, F, F, F) {
    let t11883 = F::new(1.0) / t11882;
    let t12726 = t848 * t1159;
    let t12935 = t862 * t182;
    let t13067 = t360 * t1016;
    let t13287 = t171 * t1083;
    (t11883, t12726, t12935, t13067, t13287)
}

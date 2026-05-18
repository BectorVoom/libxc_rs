//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1062/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1062<F: Float>(t579: F, t86648: F, t86686: F, t86891: F, t86933: F, t91: F, t16736: F, t28: F, t4714: F, t89: F, t1017: F, t78017: F) -> (F, F, F) {
    let t86937 = t91 * t579 * (t86648 + t86686 + t86891 + t86933);
    let t86942 = t89 * t28 * t16736 * t4714;
    let t86946 = t89 * t28 * t78017 * t1017;
    (t86937, t86942, t86946)
}

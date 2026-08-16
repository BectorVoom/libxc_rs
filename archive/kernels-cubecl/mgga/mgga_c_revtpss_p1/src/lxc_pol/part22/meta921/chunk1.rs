//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3143/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3143<F: Float>(t12256: F, t3617: F, t3362: F, t482: F, t12268: F, t1263: F, t12230: F, t5104: F, t3555: F, t488: F, t17807: F, t460: F) -> (F, F, F, F, F, F) {
    let t56246 = t3617 * t12256;
    let t56250 = t482 * t3362;
    let t56254 = t1263 * t12268;
    let t56265 = t5104 * t12230;
    let t56294 = t3555 * t488;
    let t56303 = t460 * t17807;
    (t56246, t56250, t56254, t56265, t56294, t56303)
}

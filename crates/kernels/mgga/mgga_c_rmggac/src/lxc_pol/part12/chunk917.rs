//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 917/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk917<F: Float>(t40965: F, t8620: F, t22: F, t235: F, t34812: F, t40978: F, t16503: F, t35039: F, t571: F, t7461: F, t34764: F, t8457: F, t16504: F, t7467: F, t3369: F, t7482: F) -> (F, F, F, F, F, F) {
    let t41735 = t8620 * t40965;
    let t41736 = 0.36366215538993788972e-1 * t41735;
    let t41738 = t235 * t34812 * t22;
    let t41739 = t41738 * t40978;
    let t41745 = t16503 * t35039 * t571 * t7461;
    let t41747 = t34764 * t8457;
    let t41751 = t16503 * t16504 * t571 * t7467;
    let t41755 = t16503 * t3369 * t571 * t7482;
    (t41736, t41739, t41745, t41747, t41751, t41755)
}
